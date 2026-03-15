use boa_engine::{
    Context, JsError, JsResult, JsString, JsValue, NativeFunction, js_string,
    object::ObjectInitializer, property::Attribute,
};
use markup5ever_rcdom::Handle;

pub struct Document {
    doc: Handle,
}

impl Document {
    pub fn init(context: &mut Context, root: Handle) -> JsResult<()> {
        let doc = Document { doc: root };
        doc.register_methods_on(context)
    }

    fn register_methods_on(&self, context: &mut Context) -> JsResult<()> {
        fn wrap_function(
            f: fn(&JsValue, &[JsValue], &mut Context, Handle) -> JsResult<JsValue>,
            doc: Handle,
        ) -> NativeFunction {
            // SAFETY: `Console` doesn't contain types that need tracing.
            unsafe {
                NativeFunction::from_closure(move |this, args, context| {
                    f(this, args, context, doc.clone())
                })
            }
        }

        let obj = ObjectInitializer::new(context)
            .function(
                wrap_function(Self::create_element, self.doc.clone()),
                js_string!("createElement"),
                1,
            )
            .property(js_string!("body"), js_string!("test"), Default::default())
            .build();

        context.register_global_property(js_string!("document"), obj, Attribute::empty())
    }

    fn create_element(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
        doc: Handle,
    ) -> Result<JsValue, JsError> {
        Ok(JsString::from("test").into())
    }
}
