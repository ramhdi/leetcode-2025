#pragma once
#include <iostream>
#include <vector>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* next) : val(x), next(next) {}
};

ListNode* createLinkedList(vector<int>& arr) {
    ListNode* dummy = new ListNode();
    ListNode* tail = dummy;
    for (int val : arr) {
        tail->next = new ListNode(val);
        tail = tail->next;
    }
    return dummy->next;
}

void printLinkedList(ListNode* head) {
    ListNode* curr = head;
    while (curr != nullptr) {
        cout << curr->val << " -> ";
        curr = curr->next;
    }
    cout << "NULL" << endl;
}
