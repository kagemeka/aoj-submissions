from __future__ import annotations

import asyncio
import os

import aiohttp
import filesystem.path
import selext.webdriver
import aoj

ROOT_DIRECTORY = "jp.ac.u-aizu.onlinejudge"


def _get_path(submission: aoj.Submission) -> str:
    ext = aoj.get_extension(submission.language)
    return f"{ROOT_DIRECTORY}/{submission.problem_id}/{submission.id}.{ext}"


async def _save(
    session: aiohttp.ClientSession,
    submission: aoj.Submission,
) -> None:
    submission.code = await aoj.fetch_submission_code(session, submission.id)
    save_path = _get_path(submission)
    filesystem.path.prepare_directory(save_path)
    with open(save_path, mode="w") as f:
        f.write(submission.code)


async def main() -> None:
    async with aiohttp.ClientSession() as session:
        tasks = []
        with selext.webdriver.create_chrome_driver(headless=True) as driver:
            for submission in aoj.fetch_submissions(driver, "kagemeka"):
                print(submission)
                if os.path.exists(_get_path(submission)):
                    continue
                tasks.append(asyncio.create_task(_save(session, submission)))
                await asyncio.sleep(0)
        await asyncio.gather(*tasks)


if __name__ == "__main__":
    asyncio.run(main())
