#![crate_name = "union_find"]
#![crate_type="lib"]

/**
Also known as the Disjoint-Set data structure.

Some information on UnionFind on [Wikipedia](http://en.wikipedia.org/wiki/Disjoint-set_data_structure).

Creating a set:

 ```rust
 use union_find::UnionFind;
// Create a `UnionFind` node. In order to be useful, a node must be mutable.
let mut x = UnionFind::make_set(1u);
let (mut y, mut z) = (UnionFind::make_set(1u), UnionFind::make_set(2u));
// Exploring behaivor.
assert!(x.value == 1u);
assert!(x.value == y.value);
assert!(x == y); // Gotcha! Use pointers if you need uniques.
assert!(x.value != z.value);
assert!(x != z);
assert!(x.parent == None);
 ```

 Union two sets:

 ```rust
 use union_find::UnionFind;
 // Some nodes.
 let (mut x, mut y, mut z) = (
  UnionFind::make_set("Foo"),
  UnionFind::make_set("Bar"),
  UnionFind::make_set("Baz"));
 // They are both canonical.
 assert!(x.parent == None);
 assert!(y.parent == None);
 // Union `y` with `x`.
 x.union(&mut y);
 ```

 Finding the canonical node of a set:

 ```rust
 use union_find::UnionFind;
 let (mut x, mut y, mut z) = (
  UnionFind::make_set("Foo"),
  UnionFind::make_set("Bar"),
  UnionFind::make_set("Baz"));
x.clone().union(&mut y);
// Check relationships.
assert!(y.clone().find() == x);
assert!(y.clone().find() == x.clone().find());
assert!(y.find() != z.find());
 ```

 */
#[deriving(Clone, PartialEq, Show)]
pub struct UnionFind<T> {
    /** The value of the node. */
    pub value: T,
    /**  Some(parent) for a leaf, or None for a canonical node.*/
    pub parent: Option<Box<UnionFind<T>>>
}

/**
 * The UnionFind data structure. This is a node that contains a value, and a reference to it's parent, if it has one.
 * All UnionFind nodes must be of a homogenious type.
 */
impl<T> UnionFind<T> {
    /**  Encapsulates a `value` into a `UnionFind` node. It's parent is set to `None`, meaning it's a canonical node. */
    pub fn make_set(value: T) -> UnionFind<T> {
        UnionFind { value: value, parent: None }
    }

    // There's no reason to do path compression when you can just forget about one of the values :P
    pub fn find(self) -> UnionFind<T> {
        match self.parent {
            Some(p) => p.find(),
            None => self
        }
    }

    /** Union two `UnionFind` data structures together. */
    pub fn union(self, other: &mut UnionFind<T>) {
        other.parent = Some(box self);
    }
}

#[test]
fn can_create () {
    // Create with integer.
    let int_node = UnionFind::make_set(1u);
    assert_eq!(int_node.value, 1u);
    // With String
    let string_node = UnionFind::make_set("Foo".to_string());
    assert_eq!(string_node.value, "Foo".to_string());
}

#[test]
fn can_union () {
    let one = UnionFind::make_set(1u);
    let mut two = UnionFind::make_set(2u);
    one.clone().union(&mut two);
    assert_eq!(two.find(), one);
}

#[test]
fn can_find () {
    let one = UnionFind::make_set(1u);
    let mut two = UnionFind::make_set(2u);
    // Does it find on bare?
    assert_eq!(one.clone().find().value, one.value);
    one.clone().union(&mut two);
    // Does it find the parent correctly?
    assert_eq!(two.find().value, one.value);
    assert_eq!(one.clone().find().value, one.value);
}
