// Node - a basic building block of the LinkedList
pub struct Node {
   pub data: i32,
   pub next: Option<Box<Node>>,
}

// A LinkedList struct gives you a head pointer
pub struct LinkedList {
    pub head: Option<Box<Node>>,
}

// implementation for the LinkedList Struct
impl LinkedList {

	// Constructor
    pub fn new() -> Self {
        LinkedList { head: None }
    }

	// Inserting some value in a LinedList
    pub fn insert_head(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

	// Display the address of each node in the LinkedList
    pub fn display_address(&self) {
    	let mut current = &self.head;
     	while let Some(node)=current {
			println!("{:p}" , &node.data);
			current = &node.next
		}
    }	

	//Print All the elements of the LinkedList
    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} ", node.data);
            current = &node.next;
        }
        println!();
    }

	//Get the Length of LinkedList
    pub fn get_length(&self) -> i32 {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
    
    // Get the address of the head of the LinkedList
    pub fn get_head(&self) -> i32 {
    	let current = &self.head;
   		if let Some(node) = current {
   			return node.data
   		} 
   		else {
   			todo!{};
   		}
    }
    
    //Insert at a specific index of a LinkedList
    pub fn insert_at(&mut self, index: usize, data: i32) {
        if index == 0 {
            self.insert_head(data); // Reuse the existing insert method for head insertion
            return;
        }

        let mut current = &mut self.head;
        let mut i = 0;
        while let Some(ref mut node) = current {
            if i == index - 1 {
                let new_node = Box::new(Node {
                    data,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return;
            }
            current = &mut node.next;
            i += 1;
        }
    }

    // Insert a node at the end of the list
    pub fn insert_tail(&mut self, data: i32) {
        let new_node = Box::new(Node { data, next: None });
        let mut current = &mut self.head;
        loop {
            match current {
                None => {
                    *current = Some(new_node);
                    return;
                }
                Some(ref mut node) => {
                    if node.next.is_none() {
                        node.next = Some(new_node);
                        return;
                    }
                    current = &mut node.next;
                }
            }
        }
    }

    // Remove the first node of the list
    pub fn remove_head(&mut self) {
        if self.head.is_some() {
            let next = self.head.as_mut().unwrap().next.take();
            self.head = next;
        }
    }

    // Remove the last node of the list
    pub fn remove_tail(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            self.head = None;
            return;
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next.as_ref().unwrap().next.is_none() {
                node.next = None;
                return;
            }
            current = &mut node.next;
        }
    }

    // Remove the first occurrence of a specific value
    pub fn remove_value(&mut self, data: i32) {
        if self.head.is_none() {
            return;
        }

        if self.head.as_ref().unwrap().data == data {
            self.head = self.head.as_mut().unwrap().next.take();
            return;
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if let Some(ref mut next) = node.next {
                if next.data == data {
                    node.next = next.next.take();
                    return;
                }
            }
            current = &mut node.next;
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
/*
// Function to merge two sorted linked lists
fn merge<ListNode>(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;

    while let (Some(mut n1), Some(mut n2)) = (l1.as_mut(), l2.as_mut()) {
        if n1.val <= n2.val {
            current.next = l1.take();
            current = current.next.as_mut().unwrap();
            l1 = current.next.take();
        } else {
            current.next = l2.take();
            current = current.next.as_mut().unwrap();
            l2 = current.next.take();
        }
    }

    current.next = l1.or(l2);

    dummy.next
}

// Function to split the linked list into halves
fn split<ListNode>(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let mut slow = head.as_mut().map(|node| node.next.as_mut()).flatten();
    let mut fast = head.as_mut().map(|node| node.next.as_mut()).flatten();

    while let Some(mut next) = fast {
        fast = next.next.as_mut().map(|node| node.next.as_mut()).flatten();
        if let Some(mut next) = fast {
            slow = slow.unwrap().next.as_mut();
            fast = next.next.as_mut().map(|node| node.next.as_mut()).flatten();
        }
    }

    let second_half = slow.unwrap().next.take();
    (head, second_half)
}

// Merge sort function for linked list
pub fn merge_sort<ListNode>(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.as_ref().map_or(true, |node| node.next.is_none()) {
        return head;
    }

    let (first_half, second_half) = split(head);

    let first_half_sorted = merge_sort(first_half);
    let second_half_sorted = merge_sort(second_half);

    merge(first_half_sorted, second_half_sorted)
}

// Bubble Sort for linked list
pub fn bubble_sort<ListNode>(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sorted = None;
    let mut head = head;

    loop {
        let mut swapped = false;
        let mut prev = None;
        let mut current = head.take();

        while let Some(mut node) = current {
            let next_node = node.next.take();
            if let Some(mut next) = next_node {
                if node.val > next.val {
                    swapped = true;
                    node.next = next.next.take();
                    next.next = Some(node);
                    if let Some(ref mut prev_node) = prev {
                        prev_node.next = Some(next);
                    } else {
                        head = Some(next);
                    }
                } else {
                    if prev.is_none() {
                        head = Some(node);
                    }
                    prev = Some(node);
                    current = Some(next);
                }
            } else {
                if prev.is_none() {
                    head = Some(node);
                }
                prev = Some(node);
                current = None;
            }
        }

        sorted = head;
        if !swapped {
            break;
        }
    }

    sorted
}

// Selection Sort for linked list
pub fn selection_sort<ListNode>(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sorted = None;
    let mut head = head;

    while let Some(mut min_node) = head {
        let mut prev = None;
        let mut current = min_node.next.take();
        let mut min_prev = None;
        let mut min = min_node.val;

        let mut current_node = &mut current;
        let mut current_prev = &mut prev;

        while let Some(mut node) = current_node.take() {
            if node.val < min {
                min = node.val;
                min_prev = current_prev.clone();
                min_node = node;
            }
            current_prev = &mut node.next;
            current_node = &mut node.next;
        }

        if let Some(ref mut min_prev) = min_prev {
            min_prev.next = min_node.next.take();
        } else {
            head = min_node.next.take();
        }

        min_node.next = sorted;
        sorted = Some(min_node);
    }

    sorted
}

// Insertion Sort for linked list
pub fn insertion_sort<ListNode>(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sorted = None;
    let mut head = head;

    while let Some(mut node) = head {
        head = node.next.take();
        sorted = sorted_insert(sorted, node);
    }

    sorted
}

// Helper function to insert node in sorted order
pub fn sorted_insert<ListNode>(mut head: Option<Box<ListNode>>, mut node: Box<ListNode>) -> Option<Box<ListNode>> {
    if head.is_none() || node.val <= head.as_ref().unwrap().val {
        node.next = head;
        return Some(node);
    }

    let mut current = head.as_mut().unwrap();
    while let Some(next_node) = current.next.as_mut() {
        if node.val <= next_node.val {
            node.next = current.next.take();
            current.next = Some(node);
            return head;
        }
        current = next_node;
    }

    current.next = Some(node);
    head
}

// bubble sort for a vector
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}*/

/*
// selection sort for a vector
fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

// insertion sort for a vector
fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
