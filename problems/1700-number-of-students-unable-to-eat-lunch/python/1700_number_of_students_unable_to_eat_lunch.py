from typing import List


class Solution:
    def countStudents(self, students: List[int], sandwiches: List[int]) -> int:
        count: int = 0

        while len(students) != 0 and len(sandwiches) != 0:
            student: int = students[0]
            sandwich: int = sandwiches[0]

            before: str = "".join([str(s) for s in students])

            if student != sandwich:
                students.append(students.pop(0))
            else:
                students.pop(0)
                sandwiches.pop(0)

            after: str = "".join([str(s) for s in students])

            if before == after:
                count = len(before)
                break

        return count


def test(students: List[int], sandwiches: List[int]) -> None:
    result: int = Solution().countStudents(students.copy(), sandwiches.copy())
    print(
        f"""For these students and these sandwiches {result} students can't eat! \n -> students: {students} \n -> sandwiches: {sandwiches}""")


if __name__ == "__main__":
    test(students=[1, 1, 0, 0], sandwiches=[0, 1, 0, 1])
    test(students=[1, 1, 1, 0, 0, 1], sandwiches=[1, 0, 0, 0, 1, 1])
