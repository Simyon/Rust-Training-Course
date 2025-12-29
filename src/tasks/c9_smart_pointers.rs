// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

use std::cell::RefCell;
use std::rc::Rc;

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

// IMPLEMENT HERE:
pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        BinaryTreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn with_children(
        value: i32,
        left_child: BinaryTreeNode,
        right_child: BinaryTreeNode,
    ) -> Self {
        BinaryTreeNode {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn sum(&self) -> i32 {
        let mut total = self.value;
        if let Some(left) = &self.left_child {
            total += left.sum();
        }
        if let Some(right) = &self.right_child {
            total += right.sum();
        }
        total
    }
}

// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Package>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: Package) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

// IMPLEMENT HERE:
pub struct Package {
    name: String,
    dependencies: Vec<Rc<Package>>,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Package {
            name: name.to_string(),
            dependencies: vec![],
        }
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Self {
        Package { name: name.to_string(), dependencies }
    }

    pub fn list_dependencies(&self) -> Vec<String> {
        let mut visited = Vec::new();
        let mut stack = Vec::new();

        for dep in self.dependencies.iter().rev() {
            stack.push(&**dep);
        }

        while let Some(pkg) = stack.pop() {
            if visited.contains(&pkg.name) {
                continue;
            }

            visited.push(pkg.name.clone());

            for dep in pkg.dependencies.iter().rev() {
                stack.push(&**dep);
            }
        }

        visited
    }
}

#[test]
fn test_list_dependencies() {
    let package_a = Rc::new(Package::new("A"));
    let package_b = Rc::new(Package::with_dependencies("B", vec![Rc::clone(&package_a)]));
    let package_c = Rc::new(Package::with_dependencies(
        "C",
        vec![Rc::clone(&package_a), Rc::clone(&package_b)],
    ));
    let package_d = Rc::new(Package::with_dependencies(
        "D",
        vec![Rc::clone(&package_a), Rc::clone(&package_b), Rc::clone(&package_c)],
    ));

    let deps = package_d.list_dependencies();
    assert_eq!(deps.len(), 3);
    assert!(deps.contains(&"A".to_string()));
    assert!(deps.contains(&"B".to_string()));
    assert!(deps.contains(&"C".to_string()));
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

// IMPLEMENT HERE:
pub struct SharedCounter {
    value: Rc<RefCell<i32>>,
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter { value: Rc::new(RefCell::new(0)) }
    }

    pub fn increment(&self) {
        let mut value = self.value.borrow_mut();
        *value += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
