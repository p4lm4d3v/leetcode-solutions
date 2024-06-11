from typing import List


class Solution:
    def compareVersion(self, version1: str, version2: str) -> int:
        v1 = VersionParser.parse(version1)
        v2 = VersionParser.parse(version2)
        return VersionComparer.compare(v1, v2)


class Version(str):
    def __init__(self, version: str) -> None:
        self.version = version

    def __len__(self) -> int:
        return len(self.revisions)

    def __str__(self) -> str:
        return self.version

    def expand(self, new_len: int) -> None:
        if new_len == len(self):
            return

        to_add: int = new_len - len(self)
        for _ in range(to_add):
            self.version += ".0"

    @property
    def revisions(self) -> List[str]:
        return self.version.split('.')


class VersionParser:
    def parse(version: str) -> Version:
        revisions: List[str] = version.split(".")
        revisions = [rev.lstrip('0') for rev in revisions]
        revisions = ["0" if rev == "" else rev for rev in revisions]
        v = ".".join(revisions)
        return Version(v)


class VersionComparer:
    def compare(v1: Version, v2: Version) -> int:
        max_len = max(len(v1), len(v2))
        v1.expand(max_len)
        v2.expand(max_len)
        if v1.version == v2.version:
            return 0
        else:
            for i in range(max_len):
                if int(v1.revisions[i]) > int(v2.revisions[i]):
                    return 1
                elif int(v1.revisions[i]) < int(v2.revisions[i]):
                    return -1


if __name__ == "__main__":
    print(Solution().compareVersion("1.01", "1.001"))
    print(Solution().compareVersion("1.0", "1.0.0"))
    print(Solution().compareVersion("0.1", "1.1"))
