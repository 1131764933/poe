//添加编译标签,意思是两种编译的形式
#![cfg_attr(not(feature='std'),no_std)]
//描述信息
//a module for  proof of existence

//暴露组件信息
pub use pallet::*;

//定义功能模块
pub mod pallet {
    //引入对应的依赖
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::Randomness
    };
    use frame_system::pallet_prelude::*;
    use codec::{Encode,Decode};
    use sp_io::hashing::Blake2_128;

    #[derive(Encode,Decode)]
    pub struct Kitty(pub [u8;16]);
    type KittyIndex=u32;
    //定义模块配置接口
    #[pallet::config]
    pub trait Config:frame_system::Config{
        type Event: From<Event<Self>>+IsType<<Self as frame_system::Config>::Event>;
        type Randomness:Randomness<Self::hash,Self::BlockNumber>;
    }

    //定义一个结构体来承载功能模块
     #[pallet::pallet]
     #[pallet::generate_store(pub(super)trait Store)]
     pub struct Pallet<T>(_);
    

    //定义一个事件宏
    #[pallet::event]
    #[pallet::metadata(T::AccountId="AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T:Config>{
      KittyCreate(T::AccountId,KittyIndex),
      KiityTransfer(T::AccountId,T::AccountId, KittyIndex),
    }

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type kitties_count<T> =storageValue<_,u32>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type kitties<T>=StorageMap<_,Black2_128Concat,KittyIndex,Option<Kitty>,ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T:Config>=StorageMap<_,Black2_128Concat,KittyIndex,Option<T::AccountId>,ValueQuery>;

    //定义一个错误
    #[pallet::error]
    pub enum Error<T> {
        kittiesCountOverflow,
    }
 

    //定义一个可调用函数
    #[pallet::call]
    impl<T:Config> Pallet<T>{
        #[pallet::weight(0)]
        pub fn create(origin:OriginFor<T>) -> DispatchResult{
            let who=ensure_signed(origin)?
        }

        let kitty_id=match Self::kitties_count() {
            Some(id)=>{
                ensure!(id != KittyIndex::max_value(),Error::<T>::kittiesCountOverflow);
                id
            },
            None =>{
                1
            }
        };

        let dna  = Self::random_value(&who);
        kitties::<T>::insert(kitty_id,Some(Kitty(dna)));
        Owner::<T>::insert(kitty_id,Some(who.clone()));
        kittiesCount::<T>::put(kitty_id+1);
        Self::deposit_event(Event::KittyCreate(who,kitty_id));

        ok(())
        }
        #[pallet::weight(0)]

        pub fn transfer(origin:OriginFor<T>,new_owner:T::AccountId,kitty_id:KittyIndex)->DispatchResult{
            let who=ensure_signed(origin)?
            ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id),Error::<T>::NotOwner);
            Owner::<T>::insert(kitty_id,Some(new_owner.clone()));
           
            Self::deposit_event(Event::KittyTransfer(who,new_owner,kitty_id));

        ok(())
        }


        #[pallet::weight(0)]
        pub fn breed(origin:OriginFor<T>,kitty_id_1:KittyIndex,kitty_id_2:KittyIndex) -> DispatchResult{
            let who =ensure_signed(origin)?;
            ensure!(kitty_id_1 !=kitty_id_2,Error::<T>::SameParentIndex);
            let kitty1=Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty2=Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty_id=match Self::kitties_count(){
                Some(id)=>{
                    ensure!=(id!=KittyIndex::max_value(),Error::<T>::kittiesCountOverflow);
                    id
                },
                None =>{
                    1
                }
            };
            let dna_1=kitty1.0;
            let dna_2=kitty2.0;
            let selector =Self::random_value(&who);
            let mut new_dna=[0u8;16];
            for i in 0..dna_1.len(){
                new_dna[i]=(selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]);

            }

            kitties::<T>::insert(kitty_id,Some(kitty(new_dna)));
            Owner::<T>::insert(kitty_id,Some(who.clone()));
            kittiesCount::<T>::put(kitty_id+1);
            Self::deposit_event(Event::KittyCreate(who,kitty_id));
            ok(())


        }
    impl<T:Config> Pallet<T>{
        fn random_value(sender:&T::AccountId) ->[u8;16]{
            let payload=(
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(Black2_128)
        }
        }
       

    }

//接下来在runtime中的cargo.toml进行引入
