
use prelude::World;

#[cfg(feature="serialize")]
use prelude::Entity;
#[cfg(feature="serialize")]
use serde::{self, Serializer, Deserializer};

/// Group of components. Can be subgrouped into other component groups.
pub trait ComponentGroup {
    /// Components defined in this group, not a subgroup.
    fn local_components() -> Vec<&'static str>;
    /// Components defined in this group along with subgroups.
    fn components() -> Vec<&'static str>;
    /// Subgroups included in this group.
    fn subgroups() -> Vec<&'static str>;
    /// Registers the components into the world.
    fn register(&mut World);
}

/// Group of serializable components.
#[cfg(feature="serialize")]
pub trait SerializeGroup: ComponentGroup {
    /// Serializes the group of components from the world.
    fn serialize_group<S: Serializer>(world: &World, serializer: S) -> Result<S::Ok, S::Error>;
    /// Helper method for serializing the world.
    fn serialize_subgroup<S: Serializer>(world: &World, map: &mut S::SerializeMap) -> Result<(), S::Error>;
    /// Deserializes the group of components into the world.
    fn deserialize_group<D: Deserializer>(world: &mut World, entities: &[Entity], deserializer: D) -> Result<(), D::Error>;
    /// Helper method for deserializing the world.
    fn deserialize_subgroup<V>(world: &mut World, entities: &[Entity], key: String, visitor: &mut V) -> Result<Option<()>, V::Error>
        where V: serde::de::MapVisitor;
}

macro_rules! deconstruct {
    ( $( $associated:tt )* ) => {
        $( #[doc(hidden)] type $associated; )*
    }
}

/// Deconstructs the group.
pub trait GroupLocals: ComponentGroup {
    deconstruct!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
    /// Amount of associated types this group uses.
    fn used() -> usize;
}
