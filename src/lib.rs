//! Highly customizable runtime-configuration for slog-rs/slog with
//! opinionated defaults.
//! 
//! # Overview
//!
//! This crate is roughly divided into two parts:
//!
//! 1. De-/serialization of configuration files.
//! 2. Construction of a `Drain` (or `Logger`) from a configuration object.
//!
//! Both aspects can be configured, all defaults can be completely replaced.
//! 
//! Most of the functionality is centered around the [`Config`](Config) trait
//! and trait-objects of that type. This trait describes a configuration with
//! which a logger (or drain) can be built.
//!
//! ## De-/Serialization
//!
//! Serialization of configurations is straight-forward except for a small
//! detail. Configurations should always be serialized as trait-object,
//! otherwise their `type` tag will not be included during serialization and
//! thus deserialization will fail.
//! 
//! Deserialization of `Box<Config>` can be acheived by use of the `deserialize`
//! function of a [`Deserializers`](Deserializers)-registry. A default registry
//! is provided by the [`deserializers`](deserializers)-method. This default
//! registry will be used if `Box<Config>` is directly deserialized.
//! 
//! Custom deserialization can, for example, be implemented with a
//! newtype-wrapper for `Box<Config>` and a custom registry.
//!  
//! ## Building a Logger
//!
//! Constructing a logger from a [`Config`](Config) trait-object can be done
//! via a [`Factories`](Factories) registry. A default registry is provided
//! via the [`factories`](factories)-method. [`build`](build) is a
//! convenience-method using this default registry to build a `Drain`.
//! 
//! ## Customizable Features for Compile-Time Configuration
//! 
//! The configuration types and default factories supported by this crate can
//! be found in the [`ty`](ty) module and can be configured via the
//! feature-set of this crate. For each supported type exists is a
//! corresponding feature with the same name enabling support for said type.
//! By default, all types are enabled.

extern crate serde;
extern crate serde_tagged;

extern crate erased_serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate slog;
extern crate slog_async;

#[cfg(feature = "plain")]
extern crate slog_term;

extern crate chrono;


use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeSeed;

use serde_tagged::de::{BoxFnSeed, FnSeed};
use serde_tagged::util::erased::SerializeErased;

use slog_async::{Async, AsyncGuard};


pub mod common;
pub mod ty;

#[cfg(feature = "plain")]
pub use ty::plain::{Config as PlainConfig, Factory as PlainFactory};


/// The name of the field containing the type of a serialized logger
/// configuration.
///
/// Configuration-implementations must not contain a field with this name.
pub const TYPE_KEY: &str = "type";

/// All logger types supported by this crate.
///
/// The set of supported configuration types can be configured by the feature
/// set of this crate.
pub const SUPPORTED_TYPES: &[&str] = &[
    #[cfg(feature = "plain")]
    "plain",
];

/// Returns a reference to the default deserializer-stub registry.
///
/// This registry is used for deserialization of all supported configuration
/// types when no specialized `Deserialize` implementation is used.
///
/// The set of supported configuration types can be configured by the feature
/// set of this crate.
///
/// See [`Deserializers`](::Deserializers) for more information.
pub fn deserializers() -> &'static Deserializers {
    lazy_static! {
        static ref REG: Deserializers = Deserializers::default();
    }

    &REG
}

/// Returns a reference to the default `Drain` factories.
///
/// The default factories will create an `Async` drain and its `AsyncGuard`.
///
/// The set of supported configuration types can be configured by the feature
/// set of this crate.
///
/// See [`Factories`](::Factories) for more information.
pub fn factories() -> &'static Factories<(Async, AsyncGuard)> {
    lazy_static! {
        static ref REG: Factories<(Async, AsyncGuard)> = Factories::default();
    }

    &REG
}

/// Builds a `Drain` from the given `Config` using the default factories.
///
/// This will create an `Async` drain as well as its `AsyncGuard` and is
/// equivalent to `factories().build(cfg)`.
///
/// See [`factories()`](::factories) for more information.
pub fn build(cfg: &Config) -> Result<(Async, AsyncGuard), Error> {
    factories().build(cfg)
}


#[allow(unused_imports)]
#[allow(unused_mut)]
#[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
impl Default for Deserializers {
    /// Returns a registry containing default deserializers for all supported
    /// types.
    fn default() -> Self {
        use erased::DeserializeConfig;
        let mut reg = Deserializers::empty();

        #[cfg(feature = "plain")]
        reg.register("plain", PlainConfig::deserialize_config);

        reg
    }
}

#[allow(unused_mut)]
#[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
impl Default for Factories<(Async, AsyncGuard)> {
    /// Returns a registry containing default factories for all supported
    /// configuration-types.
    ///
    /// See [`ty`](::ty) for the default factories.
    fn default() -> Self {
        let mut reg = Factories::empty();

        #[cfg(feature = "plain")]
        reg.register(PlainFactory);

        reg
    }
}


/// An error that can occur when building a logger.
#[derive(Debug, Fail)]
pub enum Error {
    /// Indicates that the selected configuration is not supported.
    #[fail(display = "unsupported configuration")]
    Unsupported,

    /// An IO error.
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),

    /// An unspecified error with a message describing the failure.
    #[fail(display = "{}", _0)]
    Msg(String),
}

impl Error {
    /// Creates a new, unspecified error with the provided message.
    pub fn msg<D>(msg: &D) -> Self
    where
        D: ToString + ?Sized,
    {
        Error::Msg(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::Io(error)
    }
}


/// A configuration describing how a logger should be created.
pub trait Config: erased_serde::Serialize + 'static {
    /// The type-tag of this configuration.
    fn ty(&self) -> &'static str;

    /// The type-id of the configuration implementation.
    ///
    /// # Warning
    ///
    /// You should not implement this method manually.
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl Config {
    /// Returns `true` if the actual type of this trait-object is the same as
    /// `T`.
    pub fn is<T: Config>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }

    /// Returns some reference to the actual value of this trait object if it is
    /// of type `T`, or `None` if it is not.
    pub fn downcast_ref<T: Config>(&self) -> Option<&T> {
        if self.type_id() == TypeId::of::<T>() {
            unsafe { Some(&*(self as *const Config as *const T)) }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the actual value of this trait object
    /// if it is of type `T`, or `None` if it is not.
    pub fn downcast_mut<T: Config>(&mut self) -> Option<&mut T> {
        if self.type_id() == TypeId::of::<T>() {
            unsafe { Some(&mut *(self as *mut Config as *mut T)) }
        } else {
            None
        }
    }
}

impl<'a> Serialize for Config + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_tagged::ser::internal::serialize(
            serializer,
            TYPE_KEY,
            self.ty(),
            &SerializeErased(self),
        )
    }
}

impl<'de> Deserialize<'de> for Box<Config> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializers().deserialize(deserializer)
    }
}


/// A factory that can build a target from a configuration.
pub trait Factory {
    /// The configuration from which the target should be built.
    type Config: Config;

    /// The target type that will be built by this factory.
    type Target;

    /// Builds a `Target` from the specified configuration.
    fn build(&self, cfg: &Self::Config) -> Result<Self::Target, Error>;
}


/// A shim that adapts a `Factory` for `Config` trait-objects.
trait FactoryShim: Sync {
    /// The target type that will be built by this factory-shim.
    type Target;

    /// Builds a `Target` from the specified configuration-object.
    fn build(&self, cfg: &Config) -> Result<Self::Target, Error>;
}

/// A `FactoryShim` implementation that panics on an invalid trait-object
/// downcast.
struct Unchecked<F>(F);

impl<F: Factory + Sync> FactoryShim for Unchecked<F> {
    type Target = F::Target;

    fn build(&self, cfg: &Config) -> Result<Self::Target, Error> {
        let cfg = cfg.downcast_ref::<F::Config>().expect("invalid cast");
        self.0.build(cfg)
    }
}


/// A registry for factories.
///
/// This registry allows for a mapping from the configuration-type associated
/// with a factory-type to an instance of this factory-type. Furthermore it
/// allows the creation of a `T` from a trait-object of type `Config` in a
/// type-safe manner, by selecting the right factory for the provided `Config`
/// type.
pub struct Factories<T> {
    store: HashMap<TypeId, Box<FactoryShim<Target = T>>>,
}

impl<T> Factories<T> {
    /// Create a new, empty factory registry.
    pub fn empty() -> Self {
        Factories {
            store: HashMap::new(),
        }
    }

    /// Register the provided factory for its associated configuration type
    /// (`F::Config`).
    ///
    /// Returns `true` if the configuration type has already been associated
    /// with a factory befor this call. Any previous mapping is being replaced.
    pub fn register<F>(&mut self, factory: F) -> bool
    where
        F: Factory<Target = T> + Sync + 'static,
    {
        let shim = Box::new(Unchecked(factory));

        match self.store.insert(TypeId::of::<F::Config>(), shim) {
            Some(_) => true,
            None => false,
        }
    }

    /// Remove the provided configuration type and its associated factory from
    /// this registry.
    pub fn deregister<C>(&mut self) -> bool
    where
        C: Config + 'static,
    {
        match self.store.remove(&TypeId::of::<C>()) {
            Some(_) => true,
            None => false,
        }
    }

    /// Remove a configuration-type and its associated factory from this
    /// registry via the `TypeId` of this configuration.
    pub fn deregister_id(&mut self, id: &TypeId) {
        self.store.remove(id);
    }

    /// Return `true` if the provided configuration-type is associated with a
    /// `Factory`.
    pub fn is_registered<C>(&self) -> bool
    where
        C: Config + 'static,
    {
        self.store.contains_key(&TypeId::of::<C>())
    }

    /// Return `true` if the provided `TypeId` is associated with a `Factory`.
    pub fn is_registered_id(&self, id: &TypeId) {
        self.store.contains_key(id);
    }

    /// Clears this registry, removing all elements.
    pub fn clear(&mut self) {
        self.store.clear()
    }

    /// Build a `T` from the specified `Config`-object in a type-safe manner.
    ///
    /// The `build` method of the factory associated with the actual type of the
    /// configuration-object will be invoked for the creation of the `T` value.
    /// If no factory is associated with the configuration-type,
    /// [`Error::Unsupported`](::Error::Unsupported) will be returned.
    ///
    /// Internally, the trait-object is being casted to the actual
    /// configuration-type of the factory. If this cast fails, this function
    /// will panic. This cast should not fail, as the factory is directly
    /// registered under its associated configuration type. If this cast fails
    /// nontheless, this may indicate that either the default implementation of
    /// [`Config::type_id()`](::Config::type_id) has been manually overwritten
    /// or a collision of two `TypeId`s has occured.
    pub fn build(&self, cfg: &Config) -> Result<T, Error> {
        self.store
            .get(&cfg.type_id())
            .ok_or_else(|| Error::Unsupported)?
            .build(cfg)
    }
}


/// A registry for `DeserializeSeed` implementations for deserialization of
/// `Config` trait-objects.
///
/// This registry allows for deserialization of `Config` trait-objects based on
/// their `type` tag by providing a dynamically dispatched implementation of
/// `DeserializeSeed`.
pub struct Deserializers {
    store: BTreeMap<&'static str, BoxFnSeed<Box<Config>>>,
}

impl Deserializers {
    /// Creates a new, empty registry.
    pub fn empty() -> Self {
        Deserializers {
            store: BTreeMap::new(),
        }
    }

    /// Registers a closure as `DeserializeSeed` for the specified type-tag.
    ///
    /// Thre registered deserializer will be used for deserializing a `Config`
    /// object with the specified value as `type`.
    ///
    /// Returns the `DeserializeSeed` previously associated with the specified
    /// type-tag, or `None` if no such entry existed.
    ///
    /// This method is a convenience-wrapper around `insert`.
    pub fn register<F>(&mut self, tag: &'static str, seed: F) -> Option<BoxFnSeed<Box<Config>>>
    where
        F: FnSeed<Box<Config>> + Sync + 'static,
    {
        self.store.insert(tag, BoxFnSeed::new(seed))
    }
}

impl std::ops::Deref for Deserializers {
    type Target = BTreeMap<&'static str, BoxFnSeed<Box<Config>>>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl std::ops::DerefMut for Deserializers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<'de, 'a> DeserializeSeed<'de> for &'a Deserializers {
    type Value = Box<Config>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        serde_tagged::de::internal::deserialize(deserializer, TYPE_KEY, &self.store)
    }
}


pub mod erased {
    //! Utilities for type-erased de-/serialization.

    use super::Config;

    use erased_serde::{Deserializer, Error};
    use serde::Deserialize;


    /// A trait providing a deserialization-mehtod for `Config` trait-objects
    /// with the correct signature required for `BoxFnSeed`.
    ///
    /// This trait is automatically implemented for all types implementing
    /// `serde::Deserialize` and `Config` and should not have to be implemented
    /// manually.
    pub trait DeserializeConfig {
        /// Deserialize a `Config` trait-object with the specified type-erased
        /// `Deserializer`.
        fn deserialize_config<'de>(de: &mut Deserializer<'de>) -> Result<Box<Config>, Error>;
    }

    impl<T> DeserializeConfig for T
    where
        T: Config + for<'de> Deserialize<'de> + 'static,
    {
        fn deserialize_config<'de>(de: &mut Deserializer<'de>) -> Result<Box<Config>, Error> {
            Ok(Box::new(Self::deserialize(de)?))
        }
    }
}
