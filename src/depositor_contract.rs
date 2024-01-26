pub use depositor::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod depositor {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"DepositAlreadyExists\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"DepositAlreadyWithdrawn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"DepositDoesNotExist\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FailedInnerCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"providedLockTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minimumLockTime\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"LockTimeTooShort\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"currentTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lockTime\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TimeLockNotExpired\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroAddressNotAllowed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ZeroDepositAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"lockTime\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Restored\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"secret\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"secretHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Withdrawn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MIN_LOCK_TIME\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"secretHash_\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lockTime_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"deposits\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lockTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isWithdrawn\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"secretHash_\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"restore\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"secret_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static DEPOSITOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct Depositor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Depositor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Depositor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Depositor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Depositor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Depositor)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Depositor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPOSITOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `MIN_LOCK_TIME` (0x3ff03207) function
        pub fn min_lock_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([63, 240, 50, 7], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xeb2243f8) function
        pub fn deposit(
            &self,
            recipient: ::ethers::core::types::Address,
            secret_hash: [u8; 32],
            lock_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 34, 67, 248], (recipient, secret_hash, lock_time))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposits` (0x3d4dff7b) function
        pub fn deposits(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([61, 77, 255, 123], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `restore` (0x205e8d53) function
        pub fn restore(
            &self,
            secret_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 94, 141, 83], secret_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            secret: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], secret)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposited` event
        pub fn deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Restored` event
        pub fn restored_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RestoredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdrawn` event
        pub fn withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawnFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Depositor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DepositAlreadyExists` with signature `DepositAlreadyExists(bytes32)` and selector `0x744674ae`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DepositAlreadyExists", abi = "DepositAlreadyExists(bytes32)")]
    pub struct DepositAlreadyExists {
        pub secret_hash: [u8; 32],
    }
    ///Custom Error type `DepositAlreadyWithdrawn` with signature `DepositAlreadyWithdrawn(bytes32)` and selector `0xa9d21a9d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "DepositAlreadyWithdrawn",
        abi = "DepositAlreadyWithdrawn(bytes32)"
    )]
    pub struct DepositAlreadyWithdrawn {
        pub secret_hash: [u8; 32],
    }
    ///Custom Error type `DepositDoesNotExist` with signature `DepositDoesNotExist(bytes32)` and selector `0x22b6cc9d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DepositDoesNotExist", abi = "DepositDoesNotExist(bytes32)")]
    pub struct DepositDoesNotExist {
        pub secret_hash: [u8; 32],
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `LockTimeTooShort` with signature `LockTimeTooShort(uint256,uint256)` and selector `0x7c711159`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "LockTimeTooShort", abi = "LockTimeTooShort(uint256,uint256)")]
    pub struct LockTimeTooShort {
        pub provided_lock_time: ::ethers::core::types::U256,
        pub minimum_lock_time: ::ethers::core::types::U256,
    }
    ///Custom Error type `TimeLockNotExpired` with signature `TimeLockNotExpired(uint256,uint256)` and selector `0x66db931a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TimeLockNotExpired", abi = "TimeLockNotExpired(uint256,uint256)")]
    pub struct TimeLockNotExpired {
        pub current_time: ::ethers::core::types::U256,
        pub lock_time: ::ethers::core::types::U256,
    }
    ///Custom Error type `ZeroAddressNotAllowed` with signature `ZeroAddressNotAllowed()` and selector `0x8579befe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroAddressNotAllowed", abi = "ZeroAddressNotAllowed()")]
    pub struct ZeroAddressNotAllowed;
    ///Custom Error type `ZeroDepositAmount` with signature `ZeroDepositAmount()` and selector `0x078e1d85`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ZeroDepositAmount", abi = "ZeroDepositAmount()")]
    pub struct ZeroDepositAmount;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DepositorErrors {
        DepositAlreadyExists(DepositAlreadyExists),
        DepositAlreadyWithdrawn(DepositAlreadyWithdrawn),
        DepositDoesNotExist(DepositDoesNotExist),
        FailedInnerCall(FailedInnerCall),
        LockTimeTooShort(LockTimeTooShort),
        TimeLockNotExpired(TimeLockNotExpired),
        ZeroAddressNotAllowed(ZeroAddressNotAllowed),
        ZeroDepositAmount(ZeroDepositAmount),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DepositorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DepositAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositAlreadyExists(decoded));
            }
            if let Ok(decoded) = <DepositAlreadyWithdrawn as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositAlreadyWithdrawn(decoded));
            }
            if let Ok(decoded) = <DepositDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositDoesNotExist(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <LockTimeTooShort as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LockTimeTooShort(decoded));
            }
            if let Ok(decoded) = <TimeLockNotExpired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TimeLockNotExpired(decoded));
            }
            if let Ok(decoded) = <ZeroAddressNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroAddressNotAllowed(decoded));
            }
            if let Ok(decoded) = <ZeroDepositAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroDepositAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DepositorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DepositAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositAlreadyWithdrawn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockTimeTooShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimeLockNotExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroAddressNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DepositorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DepositAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DepositAlreadyWithdrawn as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DepositDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LockTimeTooShort as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TimeLockNotExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroAddressNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroDepositAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DepositorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositAlreadyWithdrawn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockTimeTooShort(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimeLockNotExpired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroAddressNotAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroDepositAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DepositorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DepositAlreadyExists> for DepositorErrors {
        fn from(value: DepositAlreadyExists) -> Self {
            Self::DepositAlreadyExists(value)
        }
    }
    impl ::core::convert::From<DepositAlreadyWithdrawn> for DepositorErrors {
        fn from(value: DepositAlreadyWithdrawn) -> Self {
            Self::DepositAlreadyWithdrawn(value)
        }
    }
    impl ::core::convert::From<DepositDoesNotExist> for DepositorErrors {
        fn from(value: DepositDoesNotExist) -> Self {
            Self::DepositDoesNotExist(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for DepositorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<LockTimeTooShort> for DepositorErrors {
        fn from(value: LockTimeTooShort) -> Self {
            Self::LockTimeTooShort(value)
        }
    }
    impl ::core::convert::From<TimeLockNotExpired> for DepositorErrors {
        fn from(value: TimeLockNotExpired) -> Self {
            Self::TimeLockNotExpired(value)
        }
    }
    impl ::core::convert::From<ZeroAddressNotAllowed> for DepositorErrors {
        fn from(value: ZeroAddressNotAllowed) -> Self {
            Self::ZeroAddressNotAllowed(value)
        }
    }
    impl ::core::convert::From<ZeroDepositAmount> for DepositorErrors {
        fn from(value: ZeroDepositAmount) -> Self {
            Self::ZeroDepositAmount(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Deposited",
        abi = "Deposited(address,address,uint256,uint256,bytes32)"
    )]
    pub struct DepositedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub lock_time: ::ethers::core::types::U256,
        pub secret_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Restored", abi = "Restored(address,uint256,bytes32)")]
    pub struct RestoredFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub secret_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Withdrawn", abi = "Withdrawn(address,uint256,uint256,bytes32)")]
    pub struct WithdrawnFilter {
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub secret: ::ethers::core::types::U256,
        pub secret_hash: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DepositorEvents {
        DepositedFilter(DepositedFilter),
        RestoredFilter(RestoredFilter),
        WithdrawnFilter(WithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for DepositorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositedFilter::decode_log(log) {
                return Ok(DepositorEvents::DepositedFilter(decoded));
            }
            if let Ok(decoded) = RestoredFilter::decode_log(log) {
                return Ok(DepositorEvents::RestoredFilter(decoded));
            }
            if let Ok(decoded) = WithdrawnFilter::decode_log(log) {
                return Ok(DepositorEvents::WithdrawnFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DepositorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RestoredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawnFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositedFilter> for DepositorEvents {
        fn from(value: DepositedFilter) -> Self {
            Self::DepositedFilter(value)
        }
    }
    impl ::core::convert::From<RestoredFilter> for DepositorEvents {
        fn from(value: RestoredFilter) -> Self {
            Self::RestoredFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawnFilter> for DepositorEvents {
        fn from(value: WithdrawnFilter) -> Self {
            Self::WithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `MIN_LOCK_TIME` function with signature `MIN_LOCK_TIME()` and selector `0x3ff03207`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "MIN_LOCK_TIME", abi = "MIN_LOCK_TIME()")]
    pub struct MinLockTimeCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,bytes32,uint256)` and selector `0xeb2243f8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,bytes32,uint256)")]
    pub struct DepositCall {
        pub recipient: ::ethers::core::types::Address,
        pub secret_hash: [u8; 32],
        pub lock_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deposits` function with signature `deposits(bytes32)` and selector `0x3d4dff7b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposits", abi = "deposits(bytes32)")]
    pub struct DepositsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `restore` function with signature `restore(bytes32)` and selector `0x205e8d53`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "restore", abi = "restore(bytes32)")]
    pub struct RestoreCall {
        pub secret_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub secret: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DepositorCalls {
        MinLockTime(MinLockTimeCall),
        Deposit(DepositCall),
        Deposits(DepositsCall),
        Restore(RestoreCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for DepositorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MinLockTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinLockTime(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DepositsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposits(decoded));
            }
            if let Ok(decoded) = <RestoreCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Restore(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DepositorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MinLockTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Restore(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DepositorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinLockTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Restore(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MinLockTimeCall> for DepositorCalls {
        fn from(value: MinLockTimeCall) -> Self {
            Self::MinLockTime(value)
        }
    }
    impl ::core::convert::From<DepositCall> for DepositorCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositsCall> for DepositorCalls {
        fn from(value: DepositsCall) -> Self {
            Self::Deposits(value)
        }
    }
    impl ::core::convert::From<RestoreCall> for DepositorCalls {
        fn from(value: RestoreCall) -> Self {
            Self::Restore(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for DepositorCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `MIN_LOCK_TIME` function with signature `MIN_LOCK_TIME()` and selector `0x3ff03207`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MinLockTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deposits` function with signature `deposits(bytes32)` and selector `0x3d4dff7b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DepositsReturn {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub lock_time: ::ethers::core::types::U256,
        pub is_withdrawn: bool,
    }
}
