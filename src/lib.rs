//添加编译标签,意思是两种编译的形式
#![cfg_attr(not(feature='std'),no_std)]
//描述信息
//a module for  proof of existence

//暴露组件信息
pub use pallet::*;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;
//定义功能模块
pub mod pallet {
    //引入对应的依赖
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;

    use sp_std::vec::Vec;
    //定义模块配置接口
    #[pallet::config]
    pub trait Config:frame_system::Config{
        type Event: From<Event<Self>>+IsType<<Self as frame_system::Config>::Event>;
    }

    //定义一个结构体来承载功能模块
     #[pallet::pallet]
     #[pallet::generate_store(pub(super)trait Store)]
     pub struct Pallet<T>(_);
    //定义一个存储单元

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T:Config>=StorageMap<
    _,
    Blake2_128Concat,
    Vec<u8>,
    (T::AccountId,T::BlockNumber)
    >;

    //定义一个事件宏
    #[pallet::event]
    #[pallet::metadata(T::AccountId="AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T:Config>{
        ClaimCreated(T::AccountId,Vec<u8>),
        ClaimRevoked(T::AccountId,Vec<u8>),

    }
    //定义一个错误
    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner

    }
    //定义一个特殊的功能函数
    #[pallet::hooks]
    impl<T:Config>Hooks<BlockNumberFor<T>> for Pallet<T> {}

    //定义一个可调用函数
    #[pallet::call]
    impl<T:Config> Pallet<T>{
        //定义创建存证函数
        #[pallet::weight(0)]
        pub fn create_claim(
            origin:OriginFor<T>,
            claim:Vec<u8>
        )->DispatchResultWithPostInfo{
            let sender=ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);

            Proofs::<T>::insert(
                &claim,
                (sender.clone(),frame_system::Pallet::<T>::block_number())
            );

            //插入成功触发事件
            Self::deposit_event(Event::ClaimCreated(sender,claim));
            Ok(().into())
        }

        //下面定义一个销毁存证的函数
        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin:OriginFor<T>,
            claim:Vec<u8>)->DispatchResultWithPostInfo{
                let sender=ensure_signed(origin)?;
                 
                (owner,_)=Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

                ensure!(owner==sender,Error::<T>::NotClaimOwner);
                Proofs::<T>::remove(&claim);

                Self::deposit_event(Event::ClaimRevoked(sender,claim));
                Ok(().info())
            }

    }
}
//接下来在runtime中的cargo.toml进行引入
