use std::net::Ipv4Addr;

use uuid::Uuid;

pub struct AwsClient {/* 適宜必要な情報を持たせてください． */}

impl AwsClient {
    pub async fn new() -> Result<Self, anyhow::Error> {
        todo!()
    }
    pub async fn create_instance(&mut self, _instance_id: Uuid) -> Result<Ipv4Addr, anyhow::Error> {
        /*
         * instance_id は judge 側の持つインスタンス識別子です．
         * - exec -> judge の http リクエストにこの instance_id を載せて，どのインスタンスが準備完了になったか判別できるようにしたいです．
         * - judge が共有ディレクトリの /foo/bar をマウントしているとき，exec が共有ディレクトリの /foo/bar/{instance_id} をマウントしてほしいです．
         */
        todo!()
    }
    pub async fn terminate_instance(&mut self, _instance_id: Uuid) -> Result<(), anyhow::Error> {
        todo!()
    }
}
