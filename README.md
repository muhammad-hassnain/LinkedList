# helper
A helper library containing a LinkedLIst Implementation, various sort functions and other helper functions for help in various programming tasks. 

# Rust LinkedList Implementation

This repository contains a Rust implementation of a singly linked list, providing basic operations such as insertion at various positions, deletion, and traversal. The implementation is designed to help understand the inner workings of linked lists in a systems programming context using Rust's ownership and type safety features.

## Features

- Insert elements at the head, tail, or any arbitrary position in the list.
- Remove elements from the head, tail, or remove a specific value from the list.
- Traverse the list to print all elements or display the address of each node.
- Get the length of the list or the address of the head node's data.

## Structs

### Node

A single node of the linked list containing:
- `data: i32` - The integer data stored in the node.
- `next: Option<Box<Node>>` - A pointer to the next node in the list (if any).

### LinkedList

The linked list itself, which maintains:
- `head: Option<Box<Node>>` - A pointer to the first node in the list (if any).

## Methods

### LinkedList::new() -> Self

Constructs a new, empty `LinkedList`.

### insert_head(&mut self, data: i32)

Inserts a new node with the provided data at the beginning of the list.

### display_address(&self)

Displays the memory address of each node in the list.

### display(&self)

Prints all the elements of the linked list.

### get_length(&self) -> i32

Returns the length of the linked list.

### get_address(&self) -> &i32

Gets the address of the head node's data.

### insert_at(&mut self, index: usize, data: i32)

Inserts a new node with the provided data at the specified index.

### insert_tail(&mut self, data: i32)

Inserts a new node with the provided data at the end of the list.

### remove_head(&mut self)

Removes the first node of the list.

### remove_tail(&mut self)

Removes the last node of the list.

### remove_value(&mut self, data: i32)

Removes the first occurrence of a node with the specified data.

## Example Usage

```rust
let mut list = LinkedList::new();
list.insert_head(1);
list.insert_tail(2);
list.insert_at(1, 3); // List: 1 -> 3 -> 2

list.display(); // Output: 1 3 2
println!("Length: {}", list.get_length()); // Output: Length: 3

list.remove_head(); // Removes the head, list: 3 -> 2
list.remove_value(2); // Removes the node with data 2, list: 3
list.remove_tail(); // The list is now empty

