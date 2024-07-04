from typing import List, Optional, Self


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self) -> str:
        s: str = ""
        current: ListNode = self

        while current is not None:
            s += str(current.val)
            current = current.next

        return s


class Solution:
    def addTwoNumbers(
        self, l1: Optional[ListNode], l2: Optional[ListNode]
    ) -> Optional[ListNode]:
        current1: ListNode = l1
        current2: ListNode = l2

        carry: int = 0
        values: List[ListNode] = []

        while current1 is not None or current2 is not None:
            val1: int = 0 if current1 is None else current1.val
            val2: int = 0 if current2 is None else current2.val
            value: int = val1 + val2 + carry

            carry = value // 10
            value = value % 10
            values.append(value)

            current1 = current1 if current1 is None else current1.next
            current2 = current2 if current2 is None else current2.next

            if current1 is None and current2 is None and carry != 0:
                values.append(carry)

        return self.linkedListFromValues(values)

    def linkedListFromValues(self: Self, values: List[int]) -> Optional[ListNode]:
        result: ListNode = ListNode(values[0])
        current: ListNode = result

        for i in range(1, len(values)):
            current.next = ListNode(values[i])
            current = current.next

        return result


def linkedListFromValuesTest(a: List[int], solution: ListNode) -> None:
    b: ListNode = Solution().linkedListFromValues(a)
    if str(b) == str(solution):
        print(f" + 'Test {b} == {solution}' passed!")
    else:
        print(f" - 'Test {b} == {solution}' failed!")


def addTwoNumberTest(l1: List[int], l2: List[int], solution: List[int]) -> None:
    result: ListNode = Solution().addTwoNumbers(
        Solution().linkedListFromValues(l1),
        Solution().linkedListFromValues(l2),
    )
    solution: ListNode = Solution().linkedListFromValues(solution)
    if str(result) == str(solution):
        print(f" + 'Test {result} == {solution}' passed!")
    else:
        print(f" - Test '{result} == {solution}' failed!")


if __name__ == "__main__":
    linkedListFromValuesTest([2, 4, 3], ListNode(2, ListNode(4, ListNode(3))))
    linkedListFromValuesTest([5, 6, 4], ListNode(5, ListNode(6, ListNode(4))))
    linkedListFromValuesTest([7, 0, 8], ListNode(7, ListNode(0, ListNode(8))))
    addTwoNumberTest(
        l1=[2, 4, 3],
        l2=[5, 6, 4],
        solution=[7, 0, 8],
    )

    print()

    linkedListFromValuesTest(
        [9, 9, 9, 9, 9, 9, 9],
        ListNode(
            9,
            ListNode(
                9, ListNode(9, ListNode(9, ListNode(9, ListNode(9, ListNode(9)))))
            ),
        ),
    )
    linkedListFromValuesTest(
        [9, 9, 9, 9], ListNode(9, ListNode(9, ListNode(9, ListNode(9))))
    )
    linkedListFromValuesTest(
        [8, 9, 9, 9, 0, 0, 0, 1],
        ListNode(
            8,
            ListNode(
                9,
                ListNode(
                    9, ListNode(9, ListNode(0, ListNode(0, ListNode(0, ListNode(1)))))
                ),
            ),
        ),
    )
    addTwoNumberTest(
        l1=[9, 9, 9, 9, 9, 9, 9],
        l2=[9, 9, 9, 9],
        solution=[8, 9, 9, 9, 0, 0, 0, 1],
    )
