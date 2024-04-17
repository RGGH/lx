# Linked List 

## Here is an explanation of the most tricky part of the code, the length function: 

Function Purpose:

This function, named length, calculates the length of a linked list. In simpler terms, it counts the number of nodes present in the list.

Pseudocode:

Initialize variables:

len: This variable is used to store the count of nodes. We initialize it to 0.
n: This variable is a pointer that will traverse the linked list. We initialize it to point to the head of the list (self.head).
Loop through the linked list:

We use a while loop to iterate through the linked list as long as the current node (n) is not None (i.e., it doesn't point to the end of the list).
Inside the loop:
We increment the len counter by 1.
We move the n pointer to the next node in the list using n = node.next.
Return the length:

After the loop finishes iterating through all the nodes, the len variable will hold the total number of nodes in the list.
We return the len variable.
English Explanation:

The length function first starts with two variables: len to keep track of the number of nodes encountered and n as a pointer to iterate through the linked list. The pointer n is initialized to point to the head node (self.head).

Then, a while loop is used to traverse the linked list. As long as n is not pointing to None (i.e., not at the end of the list), the loop continues. Inside the loop, we increment the len counter by 1 to account for the current node. We also move the n pointer to the next node in the list using n = node.next.

Once the loop finishes iterating through all the nodes, the len variable will hold the total count of nodes in the linked list. Finally, the function returns the len variable, providing the length of the linked list.

---
## so why do we need *len* AND n? can't we just update *len* as we go ?

Conceptually, you could update len directly within the loop without needing an extra variable n. Here's why n is still useful in practice:

Readability: Using a separate variable n for traversal improves code readability. It makes it clearer that you're iterating through the list and not modifying the original head pointer in the process.

Immutability:  In some programming languages, linked list functions might be designed to be immutable, meaning they shouldn't change the original list structure.  Having a separate n pointer allows us to traverse the list without modifying the head reference.

Potential Optimizations: While it might not be a major factor for short linked lists, using a separate pointer n can allow for some minor optimizations in certain scenarios. For example, if the linked list implementation involves complex node structures, keeping a separate pointer for traversal might avoid unnecessary copying of the entire node data during each iteration.

Overall, while updating len directly within the loop might work functionally, using a separate n pointer provides advantages in terms of readability, potential immutability, and minor performance optimizations in specific cases.
