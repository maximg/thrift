/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements. See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership. The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied. See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use std::io;
use std::net::{TcpListener, TcpStream};
use transport::Transport;
use bufstream::BufStream;

pub trait TransportServer {
    type Transport: Transport;

    fn accept(&self) -> io::Result<Self::Transport>;
}

impl TransportServer for TcpListener {
    type Transport = TcpStream;

    fn accept(&self) -> io::Result<TcpStream> {
        self.accept().map(|res| res.0)
    }
}

pub struct BufferedTransportServer<T: TransportServer>(pub T);

impl<T> TransportServer for BufferedTransportServer<T>
where T: TransportServer,
      // FIXME: This bound is redundant and should be removed when
      // rust 1.1 stable is released.
      T::Transport: Transport {
    type Transport = BufStream<T::Transport>;

    fn accept(&self) -> io::Result<Self::Transport> {
         self.0.accept().map(|res| BufStream::new(res))
    }
}

