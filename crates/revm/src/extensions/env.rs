
pub trait ExtTxEnv {
    type TxAttributes;

    fn get_curr_tx(&self) -> Self::TxAttributes;

    fn is_valid(&self, attrs: Self::TxAttributes) -> bool;
}

pub trait ExtBlockEnv {
    type BlockAttributes;
    type BlockNum;
    fn get_block(&self, num: Self::BlockNum) -> Self::BlockAttributes;

    fn is_valid_block(&self, attrs: Self::BlockAttributes) -> bool;

}

pub trait ExtChainCfg {
    type CfgParams: ChainIdentifiable;

    fn get_cfg(&self) -> Self::CfgParams;

    fn get_chain_id(&self) -> <<Self as ExtChainCfg>::CfgParams as ChainIdentifiable>::ChainId;

    fn get_fork(&self) -> <<Self as ExtChainCfg>::CfgParams as ChainIdentifiable>::ChainId;

    fn is_valid_cfg(&self, cfg: Self::CfgParams) -> bool;
}

pub trait ChainIdentifiable {
    type ChainId;
    type ForkId;
}