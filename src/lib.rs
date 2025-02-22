//! # bevy_minecraft
//!
//! A toolkit for building Minecraft servers in `bevy`.
//! Currently in heavy development!
//!

use std::io;

use async_net::TcpListener;
use bevy::{prelude::*, tasks::IoTaskPool};
use futures_lite::{AsyncReadExt as _, AsyncWriteExt as _};

#[derive(Debug)]
pub struct MinecraftPlugin {
    pub port: u16,
}

impl Default for MinecraftPlugin {
    fn default() -> Self {
        Self { port: 25565 }
    }
}

impl Plugin for MinecraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_echo_server(self.port));
    }
}

fn start_echo_server(port: u16) -> impl FnMut() {
    move || {
        IoTaskPool::get()
            .spawn(async move {
                let listener = TcpListener::bind(("0.0.0.0", port)).await?;

                loop {
                    let (socket, addr) = listener.accept().await?;
                    info!("Accepted connection from {addr}");

                    let (mut reader, mut writer) = futures_lite::io::split(socket);

                    IoTaskPool::get()
                        .spawn(async move {
                            let mut buf = [0u8; 1024];

                            while let Ok(n) = reader.read(&mut buf).await {
                                if n == 0 {
                                    info!("Connection closed by the client");
                                    break;
                                }
                                info!(byte_count = n, buf = &buf[..n], "Received message!");
                                writer.write_all(&buf[0..n]).await?;
                            }

                            Ok::<(), io::Error>(())
                        })
                        .detach();
                }

                // we need this for the `?` to work
                #[allow(unreachable_code)]
                Ok::<(), io::Error>(())
            })
            .detach();
    }
}
