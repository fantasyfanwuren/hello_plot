use log::{debug, error, info, warn};
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time;

#[derive(Deserialize, Serialize, Debug)]
struct HDisk {
    target_dir: String,
    limit_num: i32,
    limit_rate: f32,
}

#[derive(Deserialize, Serialize, Debug)]
struct UserSet {
    sourse_dir: String,
    hdisks: Vec<HDisk>,
}

#[derive(Debug)]
struct HDiskInfo {
    target_dir: String,
    limit_num: i32,
    current_rate: f32,
    transferd: f32,
    finished_num: i32,
    state: String,
}

struct TotalInfo {
    hdisk_infos: Vec<HDiskInfo>,
}

impl TotalInfo {
    fn show(&self) {
        // 清理屏幕
        print!("\x1b[2J");
        print!("\x1b[H");

        // 基础信息展示
        println!(
            "本脚本开源免费,我的捐赠地址:xch1uhjvk0qm4sth2p3xlf0pv9x00w65ttfjh9h5eerynusnh7yuslqqhf2nm2"
        );

        // 表格展示
        let mut table = Table::new();

        // 表头
        table.add_row(Row::new(vec![
            Cell::new("线程ID"),
            Cell::new("最终目录"),
            Cell::new("限制分发数量(个)"),
            Cell::new("分发完成数量(个)"),
            Cell::new("当前传输速率(M/s)"),
            Cell::new("已完成传输量(M)"),
            Cell::new("当前状态"),
        ]));

        // 内容
        for (id, item) in self.hdisk_infos.iter().enumerate() {
            table.add_row(Row::new(vec![
                Cell::new(&id.to_string()),
                Cell::new(&item.target_dir),
                Cell::new(&item.limit_num.to_string()),
                Cell::new(&item.finished_num.to_string()),
                Cell::new(&item.current_rate.to_string()),
                Cell::new(&item.transferd.to_string()),
                Cell::new(&item.state),
            ]));
        }

        table.print_tty(true).unwrap();
    }

    fn finished_add_one(&mut self, id: usize) {
        self.hdisk_infos[id].finished_num += 1;
        self.hdisk_infos[id].current_rate = 0.0;
        self.hdisk_infos[id].transferd = 0.0;
        if self.hdisk_infos[id].finished_num >= self.hdisk_infos[id].limit_num {
            self.hdisk_infos[id].state = "已完成传输任务".to_string();
        } else {
            self.hdisk_infos[id].state = "正在检测新的plot文件中...".to_string();
        }

        self.show();
    }

    fn change_to_transfering(&mut self, filename: &str, id: usize) {
        self.hdisk_infos[id].state = format!("传输中：{}..", &filename[..30]);
        self.show();
    }

    fn update_current_rate(&mut self, rate: f32, transferd: f32, id: usize) {
        self.hdisk_infos[id].current_rate = rate;
        self.hdisk_infos[id].transferd = transferd;
        self.show();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    // 获取用户设置
    let userset: UserSet = {
        let user_set_str = fs::read_to_string("./userset.json")?;
        serde_json::from_str(&user_set_str)?
    };
    debug!("获取用户设置：{:?}", userset);
    // 检测用户设置
    check_userset(&userset)?;

    // 扫描并清理最终目录
    for item in userset.hdisks.iter() {
        let final_path = &item.target_dir;
        remove_temp(final_path).await?;
        info!("清理上次未传输完成的tmp文件:{final_path}");
    }

    // 得到用户设置的源目录
    let sourse_dir = userset.sourse_dir;

    // 设置给用户展示的信息
    let hdisk_infos: Vec<HDiskInfo> = userset
        .hdisks
        .iter()
        .map(|x| HDiskInfo {
            target_dir: x.target_dir.clone(),
            limit_num: x.limit_num,
            current_rate: 0.0,
            transferd: 0.0,
            finished_num: 0,
            state: "正在检测新的plot文件...".to_string(),
        })
        .collect();

    let total_info = TotalInfo { hdisk_infos };
    total_info.show();

    let total_info = Arc::new(Mutex::new(total_info));
    let transfered_file = Arc::new(Mutex::new(vec![]));

    // 执行多线程
    let mut handles = vec![];
    for (id, h_disk) in userset.hdisks.into_iter().enumerate() {
        let total_info = Arc::clone(&total_info);
        let transfered_file = Arc::clone(&transfered_file);
        let sourse_dir = sourse_dir.clone();

        let handle = task::spawn(async move {
            // 计算总数
            let mut total_num = 0;
            'continue_up: loop {
                time::sleep(time::Duration::from_secs(10)).await;
                // 查看目录下所有的文件
                let plot_names = scan_plot(&sourse_dir).await.unwrap();
                // println!("id: {id}:{:?}", plot_names);
                debug!("线程{id}:扫描sourse_dir结果:{:?}", plot_names);

                // 选择要转移的文件名
                let mut file_name = "".to_owned();
                {
                    debug!("线程{}:等待transfered_file和total_info,用于判断优先级", id);
                    let mut transfered_file = transfered_file.lock().unwrap();
                    let mut total_info = total_info.lock().unwrap();
                    debug!("线程{id}:已经获取transfered_file和total_info所有权,用于判断优先级");

                    'for_up: for name in plot_names {
                        if !transfered_file.contains(&name) {
                            // 判断是否存在最优线程
                            let current_remain_num = total_info.hdisk_infos[id].limit_num
                                - total_info.hdisk_infos[id].finished_num;
                            for item in total_info.hdisk_infos.iter() {
                                let item_remain_num = item.limit_num - item.finished_num;
                                if item_remain_num > current_remain_num
                                    && item.state.contains("正在检测")
                                {
                                    debug!("线程{id}:选择出让本次捕获的{name}。因为{}目前空闲，且空间更充足",item.target_dir);
                                    drop(total_info);
                                    drop(transfered_file);
                                    continue 'continue_up;
                                }
                            }

                            total_info.change_to_transfering(&name, id);
                            transfered_file.push(name.clone());
                            debug!("线程{id}:成功捕获{name}");
                            file_name = name.clone();
                            break 'for_up;
                        }
                    }
                    drop(total_info);
                    drop(transfered_file);
                }

                // 判断是否找到要转移的文件
                if !file_name.is_empty() {
                    // 转移文件
                    transfer_file(
                        &file_name,
                        &sourse_dir,
                        &h_disk.target_dir,
                        h_disk.limit_rate,
                        Arc::clone(&total_info),
                        id,
                    )
                    .await
                    .unwrap();
                    // 计数+1
                    total_num = total_num + 1;
                    // println!("{}>>>>>当前转移个数:{}", h_disk.target_dir, total_num)
                    {
                        debug!("线程{}:等待total_info,来让完成数量+1", id);
                        let mut total_info = total_info.lock().unwrap();
                        total_info.finished_add_one(id);
                        debug!("线程{}:已得到total_info所有权,完成数量+1", id);
                        drop(total_info);
                    }
                }

                if total_num >= h_disk.limit_num {
                    break;
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    Ok(())
}

async fn remove_temp(final_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let files = fs::read_dir(final_dir)?;
    for file in files {
        let file_path = file?.path();
        if let Some(extension) = file_path.extension() {
            if extension == "temp" {
                fs::remove_file(file_path)?
            }
        }
    }
    Ok(())
}

async fn scan_plot(source_dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result = vec![];
    let files = fs::read_dir(source_dir)?;
    for file in files {
        let file_path = file?.path();
        if let Some(extension) = file_path.extension() {
            if extension == "plot" {
                let file_path = file_path.file_name().unwrap().to_str().unwrap().to_owned();
                result.push(file_path);
            }
        }
    }
    Ok(result)
}

fn check_userset(the_set: &UserSet) -> Result<(), Box<dyn std::error::Error>> {
    let sourse_dir = &the_set.sourse_dir;
    let sourse_dir_path = Path::new(sourse_dir);
    if !sourse_dir_path.is_dir() {
        let err_msg = format!("err:{}不是一个文件夹!", sourse_dir);
        error!("{err_msg}");
        return Err(err_msg.into());
    }
    for item in the_set.hdisks.iter() {
        let target_path = Path::new(&item.target_dir);
        if !target_path.is_dir() {
            let err_msg = format!("err:{}不是一个文件夹!", item.target_dir);
            error!("{err_msg}");
            return Err(err_msg.into());
        }
    }
    info!("检查用户设置通过");
    Ok(())
}

async fn transfer_file(
    filename: &str,
    source_dir: &str,
    target_dir: &str,
    limit_transfer_rate: f32,
    show_info: Arc<Mutex<TotalInfo>>,
    id: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("线程{id}:开始从{source_dir}转移{filename}到{target_dir},限速{limit_transfer_rate}m/s");
    let source_path = Path::new(source_dir).join(filename);
    let temp_name = format!("{}.temp", filename);
    let target_path = Path::new(target_dir).join(&temp_name);

    let mut source_file = fs::File::open(source_path)?;
    let mut target_file = fs::File::create(target_path)?;

    let mut buffer = [0; 1024 * 400];
    let mut total_bytes = 0;
    let start_time = time::Instant::now();
    // let wait_time = time::Duration::from_millis(1);
    let mut read_time = 0;
    loop {
        let bytes_read = source_file.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        target_file.write_all(&buffer[..bytes_read])?;
        // time::sleep(wait_time).await;

        total_bytes += bytes_read;

        let elapsed_time = start_time.elapsed().as_secs_f32();
        let transfer_rate = total_bytes as f32 / elapsed_time / 1024.0 / 1024.0;

        if read_time % 4000 == 0 {
            {
                debug!("线程{}:等待show_inf来更新速率", id);
                let mut show_info = show_info.lock().unwrap();
                show_info.update_current_rate(
                    transfer_rate,
                    total_bytes as f32 / 1024.0 / 1024.0,
                    id,
                );
                debug!("线程{}:已获取show_info的所有权,更新了速率和传输量", id);
                drop(show_info);
            }
        }

        if transfer_rate > limit_transfer_rate {
            let sleep_time =
                time::Duration::from_millis((bytes_read as f32 / 100.0 / 1024.0 * 1000.0) as u64);
            time::sleep(sleep_time).await;
        }
        read_time += 1;
    }

    // 删除源文件
    std::fs::remove_file(format!("{}/{}", source_dir, filename))?;
    info!("线程{id}:{}删除成功。", filename);

    // 修改目标文件为plot后缀
    let temp_path = format!("{}/{}", target_dir, temp_name);
    let final_path = format!("{}/{}", target_dir, filename);
    info!("线程{id}:将{}重命名为{}", temp_path, final_path);
    fs::rename(temp_path, final_path)?;
    Ok(())
}
