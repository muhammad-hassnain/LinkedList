## Important Note
This is for experimental purposes only. If you are not a study participant, you should not use it under any circumstances.


# LinkedList Implementation in Rust

This repository contains an implementation of a singly linked list in Rust. The linked list supports fundamental operations such as insertion, deletion, and traversal, offering a practical example of Rust's ownership and type safety features.

## Overview

The implementation includes two main structures: `Node` and `LinkedList`. The `Node` struct represents an individual element in the list, holding its data and a pointer to the next node. The `LinkedList` struct provides the functionality to manage the list, including methods to insert, remove, and display nodes.

## Structs

### Node

- **Fields:**
  - `data: i32`: Stores the integer data of the node.
  - `next: Option<Box<Node>>`: Points to the next node in the list, if any.

### LinkedList

- **Fields:**
  - `head: Option<Box<Node>>`: Points to the first node in the list, if the list is not empty.

## Methods

### `LinkedList::new() -> Self`

Constructs a new, empty `LinkedList`.

- **Input:** None
- **Output:** An instance of `LinkedList`.

### `LinkedList::insert_head(data: i32)`

Inserts a new node with the provided data at the beginning of the list.

- **Input:** `data: i32` - The data to be inserted.
- **Output:** None. Modifies the list in-place.

### `LinkedList::display_address()`

Displays the memory address of each node in the list. This method is useful for debugging and understanding how Rust handles memory with `Box`.

- **Input:** None
- **Output:** None. Prints the address of each node's data to the console.

### `LinkedList::display()`

Prints all the elements of the linked list in order from head to tail.

- **Input:** None
- **Output:** None. Prints the data of each node to the console, followed by a newline.

### `LinkedList::get_length() -> i32`

Calculates and returns the length of the linked list, i.e., the total number of nodes.

- **Input:** None
- **Output:** `i32` - The number of elements in the list.

### `LinkedList::get_address() -> &i32`

Gets the address of the head node's data. This method can be used to verify the internal structure or for debugging purposes.

- **Input:** None
- **Output:** `&i32` - A reference to the head node's data.

### `LinkedList::insert_at(index: usize, data: i32)`

Inserts a new node with the provided data at the specified index in the list.

- **Input:**
  - `index: usize` - The position where the new node should be inserted.
  - `data: i32` - The data for the new node.
- **Output:** None. Modifies the list in-place.

### `LinkedList::insert_tail(data: i32)`

Adds a new node with the specified data to the end of the list.

- **Input:** `data: i32` - The data for the new node.
- **Output:** None. Modifies the list in-place.

### `LinkedList::remove_head()`

Removes the first node of the list, effectively decreasing its size by one.

- **Input:** None
- **Output:** None. Modifies the list in-place.

### `LinkedList::remove_tail()`

Removes the last node of the list. This method iterates through the list to find the tail's predecessor.

- **Input:** None
- **Output:** None. Modifies the list in-place.

### `LinkedList::remove_value(data: i32)`

Searches for and removes the first node that contains the specified data.

- **Input:** `data: i32` - The data of the node to remove.
- **Output:** None. Modifies the list in-place if a matching node is found.

## Example Usage

Add the following line to your Cargo.toml. 

```
LinkedList = {git = "https://github.com/muhammad-hassnain/LinkedList.git"}
```
This is a basic usage of the library. For furthur details refer to the documentation. 

```rust

use LinkedList::LinkedList;

let mut list = LinkedList::new();
list.insert_head(1);
list.insert_tail(3);
list.insert_at(1, 2); // List: 1 -> 2 -> 3

list.display(); // Outputs: 1 2 3
println!("Length: {}", list.get_length()); // Outputs: Length: 3

list.remove_head(); // List: 2 -> 3
list.remove_value(2); // List: 3
list.remove_tail(); // The list is now empty
```
