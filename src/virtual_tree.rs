use std::any::Any;

use std::sync::Arc;

use crate::layout::FlexDirection;
use crate::Tree;

use dioxus::core::Mutations;

use dioxus::prelude::VirtualDom;

use dioxus_native_core::node_ref::*;
use dioxus_native_core::prelude::*;
use dioxus_native_core_macro::partial_derive_state;

#[derive(Clone, Default)]
pub struct DynAttribute(Option<Arc<dyn Any + Send + Sync>>);

impl FromAnyValue for DynAttribute {
    fn from_any_value(_value: &dyn std::any::Any) -> Self {
        todo!()
    }
}

#[partial_derive_state]
impl State<DynAttribute> for FlexDirection {
    // TextColor depends on the TextColor part of the parent
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    // Border does not depended on any other member in the current node
    const NODE_MASK: NodeMaskBuilder<'static> =
        // Get access to the border attribute
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&["flex_direction"]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<DynAttribute>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let direction: FlexDirection = node_view
            .attributes()
            .and_then(|mut attrs| {
                attrs.next().map(|a| {
                    if a.attribute.name == "flex_direction" {
                        let i = a.value.as_int().unwrap();
                        let n = u8::try_from(i).unwrap();
                        n.try_into().unwrap()
                    } else {
                        todo!()
                    }
                })
            })
            .unwrap_or_default();

        // check if the member has changed
        let changed = direction != *self;
        *self = direction;
        changed
    }
}

pub struct VirtualTree {
    tree: Tree,
    vdom: VirtualDom,
    rdom: RealDom<DynAttribute>,
    state: DioxusState,
}

impl VirtualTree {
    pub fn new(app: dioxus::prelude::Component) -> Self {
        let mut vdom = VirtualDom::new(app);
        let mut tree = Tree::default();
        let mut rdom = RealDom::new([FlexDirection::to_type_erased()]);
        let mut state = DioxusState::create(&mut rdom);

        let mutations = vdom.rebuild();
        apply(&mut rdom, &mut tree, &mut state, mutations);

        Self {
            tree,
            vdom,
            state,
            rdom,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.vdom.wait_for_work().await;

        let mutations = self.vdom.render_immediate();
        apply(&mut self.rdom, &mut self.tree, &mut self.state, mutations);

        Ok(())
    }
}

fn apply(
    rdom: &mut RealDom<DynAttribute>,
    tree: &mut Tree,
    state: &mut DioxusState,
    mutations: Mutations,
) {
    state.apply_mutations(rdom, mutations);

    let ctx = SendAnyMap::new();
    let (updates, changes) = rdom.update_state(ctx);

    for id in updates {
        let node = rdom.get(id).unwrap();
        tree.insert(node);
    }

    for (id, mask) in changes.iter() {
        let node = rdom.get(*id).unwrap();
        tree.update(id.clone(), node, mask.clone());
    }
}
