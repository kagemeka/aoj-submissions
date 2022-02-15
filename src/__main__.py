from __future__ import annotations

import asyncio
import dataclasses
import os

import aiohttp
import filesystem.path

_SOLUTIONS_URL = "https://judgeapi.u-aizu.ac.jp/solutions/users"
_REVIEW_URL = "https://judgeapi.u-aizu.ac.jp/reviews"

ROOT_DIRECTORY = "jp.ac.u-aizu.onlinejudge"


@dataclasses.dataclass
class SolutionSummary:
    judgeId: int
    userId: str
    problemId: str
    language: str
    version: str
    submissionDate: int
    judgeDate: int
    cpuTime: int
    memory: int
    codeSize: int
    server: int
    policy: str
    rating: int
    review: int


async def fetch_solution_list(
    session: aiohttp.ClientSession, user_id: str
) -> list[SolutionSummary]:
    url = f"{_SOLUTIONS_URL}/{user_id}"
    params = {
        "size": 100000,
    }
    async with session.get(url, params=params) as response:
        return [SolutionSummary(**data) for data in await response.json()]


@dataclasses.dataclass
class SolutionReview:
    judgeId: int
    userId: str
    problemId: str
    language: str
    submissionDate: int
    cpuTime: int
    memory: int
    policy: str
    sourceCode: str
    reviewed: int


async def fetch_solusion_review(
    session: aiohttp.ClientSession,
    judge_id: int,
) -> SolutionReview:
    url = f"{_REVIEW_URL}/{judge_id}"
    async with session.get(url) as response:
        return SolutionReview(**await response.json())


_LANGUAGE_TO_EXTENSION = {
    "C++": "cpp",
    "C": "c",
    "Java": "java",
    "C++11": "cpp",
    "C++14": "cpp",
    "C++17": "cpp",
    "C#": "cs",
    "D": "d",
    "Go": "go",
    "Ruby": "rb",
    "Rust": "rs",
    "Python": "py",
    "Python3": "py",
    "JavaScript": "js",
    "Scala": "scala",
    "Haskell": "hs",
    "OCaml": "ml",
    "PHP": "php",
    "Kotlin": "kt",
}


def get_extension(language: str) -> str:
    return _LANGUAGE_TO_EXTENSION[language]


def _get_path(solution: SolutionSummary | SolutionReview) -> str:
    ext = get_extension(solution.language)
    return f"{ROOT_DIRECTORY}/{solution.problemId}/{solution.judgeId}.{ext}"


def _save(solution: SolutionReview) -> None:
    save_path = _get_path(solution)
    filesystem.path.prepare_directory(save_path)
    with open(save_path, mode="w") as f:
        f.write(solution.sourceCode)


async def main() -> None:

    async with aiohttp.ClientSession() as session:
        solutions = await fetch_solution_list(session, "kagemeka")
        solutions = [
            solution
            for solution in solutions
            if not os.path.exists(_get_path(solution))
        ]
        tasks = [
            asyncio.create_task(
                fetch_solusion_review(session, solution.judgeId)
            )
            for solution in solutions
            if not os.path.exists(_get_path(solution))
        ]
        solution_reviews = await asyncio.gather(*tasks)
        for solution in solution_reviews:
            _save(solution)


if __name__ == "__main__":
    asyncio.run(main())
