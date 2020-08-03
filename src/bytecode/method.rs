use super::{atom::*, attribute::*, constant_pool::ConstantPool, Traveler};

use std::sync::Arc;

pub type Methods = Vec<Arc<Method>>;

const ACC_PUBLIC: U2 = 0x0001; // Declared public; may be accessed from outside its package.

const ACC_PRIVATE: U2 = 0x0002; // Declared private; accessible only within the defining class.

const ACC_PROTECTED: U2 = 0x0004; // Declared protected; may be accessed within subclasses.

const ACC_STATIC: U2 = 0x0008; // Declared static.

const ACC_FINAL: U2 = 0x0010; // Declared final; must not be overridden (§5.4.5).

const ACC_SYNCHRONIZED: U2 = 0x0020; // Declared synchronized; invocation is wrapped by a monitor use.

const ACC_BRIDGE: U2 = 0x0040; // A bridge method, generated by the compiler.

const ACC_VARARGS: U2 = 0x0080; // Declared with variable number of arguments.

const ACC_NATIVE: U2 = 0x0100; // Declared native; implemented in a language other than Java.

const ACC_ABSTRACT: U2 = 0x0400; // Declared abstract; no implementation is provided.

const ACC_STRICT: U2 = 0x0800; // Declared strictfp; floating-point mode is FPstrict.

const ACC_SYNTHETIC: U2 = 0x1000; // Declared synthetic; not present in the source code.

pub struct Method {
    pub access_flag: U2,
    pub name: String,
    pub descriptor: String,
    pub attributes: Attributes,
}

impl Traveler<Method> for Method {
    fn read<I>(seq: &mut I, constants: Option<&ConstantPool>) -> Method
    where
        I: Iterator<Item = u8>,
    {
        let access_flag = U2::read(seq, None);
        if let Some(pool) = constants {
            return Method {
                access_flag: access_flag,
                name: pool.get_str(U2::read(seq, None)).to_string(),
                descriptor: pool.get_str(U2::read(seq, None)).to_string(),
                attributes: Attributes::read(seq, Some(pool)),
            };
        }
        panic!("need constant pool to resolve methods");
    }
}

pub type CodeSegment = (
    U2,
    U2,
    Arc<Vec<u8>>,
    Arc<Vec<ExceptionHandler>>,
    Arc<Attributes>,
);

impl Method {
    pub fn get_code(&self) -> Option<CodeSegment> {
        for attr in &self.attributes {
            match attr {
                Attribute::Code(stacks, locals, code, exception, attribute) => {
                    return Some((
                        *stacks,
                        *locals,
                        Arc::clone(code),
                        Arc::clone(exception),
                        Arc::clone(attribute),
                    ));
                }
                _ => continue,
            }
        }
        return None;
    }

    pub fn get_name_and_descriptor(&self) -> (&str, &str, U2) {
        (
            self.name.as_ref(),
            self.descriptor.as_ref(),
            self.access_flag,
        )
    }

    pub fn is_public(&self) -> bool {
        self.access_flag & ACC_PUBLIC == ACC_PUBLIC
    }

    pub fn is_protected(&self) -> bool {
        self.access_flag & ACC_PROTECTED == ACC_PROTECTED
    }

    pub fn is_final(&self) -> bool {
        self.access_flag & ACC_FINAL == ACC_FINAL
    }

    pub fn is_static(&self) -> bool {
        self.access_flag & ACC_STATIC == ACC_STATIC
    }

    pub fn is_native(&self) -> bool {
        self.access_flag & ACC_NATIVE == ACC_NATIVE
    }

    pub fn is_synchronized(&self) -> bool {
        self.access_flag & ACC_SYNCHRONIZED == ACC_SYNCHRONIZED
    }
}

impl Traveler<Methods> for Methods {
    fn read<I>(seq: &mut I, constants: Option<&ConstantPool>) -> Methods
    where
        I: Iterator<Item = u8>,
    {
        let size = U2::read(seq, None);
        let mut methods = Vec::<Arc<Method>>::with_capacity(size as usize);
        for _x in 0..size {
            let method = Method::read(seq, constants);
            methods.push(Arc::new(method));
        }
        methods
    }
}

// impl Methods {
//     pub fn find(&self, name: &str, descriptor: &str) -> Option<Arc<Method>> {
//         for m in &self {
//             if m.descriptor == descriptor && m.name == name {
//                 return Some(Arc::clone(m));
//             }
//         }
//         None
//     }
// }
