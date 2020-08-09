//! Generated messages reflection support.

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::message::Message;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::name::compute_full_name;
use crate::reflect::FieldDescriptor;
use std::collections::HashMap;
use std::marker;

/// Sized to dynamic reflection operations.
pub(crate) trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<dyn Message>;
    fn default_instance(&self) -> &dyn Message;
    fn clone(&self, message: &dyn Message) -> Box<dyn Message>;
    fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool;
}

/// The only message factory implementation.
pub(crate) struct MessageFactoryImpl<M>(pub marker::PhantomData<M>);

impl<M> MessageFactory for MessageFactoryImpl<M>
where
    M: 'static + Message + Default + Clone + PartialEq,
{
    fn new_instance(&self) -> Box<dyn Message> {
        let m: M = Default::default();
        Box::new(m)
    }

    fn default_instance(&self) -> &dyn Message {
        M::default_instance() as &dyn Message
    }

    fn clone(&self, message: &dyn Message) -> Box<dyn Message> {
        let m: &M = message.downcast_ref().expect("wrong message type");
        Box::new(m.clone())
    }

    fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool {
        let a: &M = a.downcast_ref().expect("wrong message type");
        let b: &M = b.downcast_ref().expect("wrong message type");
        a == b
    }
}

#[doc(hidden)]
pub struct GeneratedMessageDescriptorData {
    index: u32,
    pub(crate) protobuf_name_to_package: &'static str,
    pub(crate) fields: Vec<FieldAccessor>,
    pub(crate) factory: &'static dyn MessageFactory,
}

impl GeneratedMessageDescriptorData {
    /// Construct a new message descriptor.
    ///
    /// This operation is called from generated code and rarely
    /// need to be called directly.
    ///
    /// This function is not a part of public API.
    #[doc(hidden)]
    pub fn new_2<M: 'static + Message + Default + Clone + PartialEq>(
        protobuf_name_to_package: &'static str,
        index: u32,
        fields: Vec<FieldAccessor>,
    ) -> GeneratedMessageDescriptorData {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        GeneratedMessageDescriptorData {
            index,
            protobuf_name_to_package,
            fields,
            factory,
        }
    }
}

pub(crate) struct GeneratedMessageDescriptor {
    pub(crate) proto: &'static DescriptorProto,

    pub(crate) full_name: String,

    pub(crate) factory: &'static dyn MessageFactory,

    pub(crate) fields: Vec<FieldDescriptor>,

    pub(crate) index_by_name: HashMap<String, usize>,
    pub(crate) index_by_name_or_json_name: HashMap<String, usize>,
    pub(crate) index_by_number: HashMap<u32, usize>,
}

impl GeneratedMessageDescriptor {
    pub fn new(
        data: GeneratedMessageDescriptorData,
        expected_index: u32,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> GeneratedMessageDescriptor {
        let GeneratedMessageDescriptorData {
            index,
            protobuf_name_to_package,
            fields,
            factory,
        } = data;

        assert!(expected_index == index);

        let (path_to_package, proto) =
            match find_message_or_enum(file_descriptor_proto, protobuf_name_to_package) {
                (path_to_package, MessageOrEnum::Message(m)) => (path_to_package, m),
                (_, MessageOrEnum::Enum(_)) => panic!("not a message"),
            };

        let mut field_proto_by_name = HashMap::new();
        for field_proto in &proto.field {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let fields: Vec<_> = fields
            .into_iter()
            .map(|f| {
                let proto = *field_proto_by_name.get(f.name).unwrap();
                FieldDescriptor::new(f, proto)
            })
            .collect();

        for (i, f) in fields.iter().enumerate() {
            assert!(index_by_number
                .insert(f.proto().get_number() as u32, i)
                .is_none());
            assert!(index_by_name
                .insert(f.proto().get_name().to_owned(), i)
                .is_none());
            assert!(index_by_name_or_json_name
                .insert(f.proto().get_name().to_owned(), i)
                .is_none());

            let json_name = f.json_name().to_owned();

            if json_name != f.proto().get_name() {
                assert!(index_by_name_or_json_name.insert(json_name, i).is_none());
            }
        }

        GeneratedMessageDescriptor {
            full_name: compute_full_name(
                file_descriptor_proto.get_package(),
                &path_to_package,
                proto.get_name(),
            ),
            fields,
            index_by_name,
            index_by_name_or_json_name,
            index_by_number,
            factory,
            proto,
        }
    }
}