"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import Tag from "./components/tag";

export interface post {
    slug: string;
    title: string;
    date: string;
    summary: string;
    tags: string[];
}

const MAX_POSTS = 5;

export default function Main() {
    const [posts, setPosts] = useState<post[]>([]);
    const [total, setTotal] = useState(1);

    useEffect(() => {

        const fetchTotals = async () => {
            const res = await fetch("/api/post/get_ports_all_count");
            const data = await res.json();
            if (data.code === 200) {
                setTotal(data.data);
            }
        }
        const fetchPosts = async () => {
            const res = await fetch("/api/post/get_list_pagination",
                {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({
                        page: 1,
                        page_size: MAX_POSTS,
                        tag: "",
                    }),
                }
            );
            const data = await res.json();
            if (data.code === 200) {
                setPosts(data.data.slice(0, MAX_POSTS));
            }
        }
            ;
        fetchTotals().then(r => fetchPosts());
    }, []
    )
        ;
    return (
        <>
            <div className="divide-y divide-gray-200 dark:divide-gray-700">
                <div className="space-y-2 pb-8 pt-6 md:space-y-5">
                    <h1 className="text-3xl font-extrabold leading-9 tracking-tight text-gray-900 dark:text-gray-100 sm:text-4xl sm:leading-10 md:text-6xl md:leading-14">
                        Latest
                    </h1>
                    <p className="text-lg leading-7 text-gray-500 dark:text-gray-400">
                        My blog with nextjs tailwind and axum
                    </p>
                </div>
                <ul className="divide-y divide-gray-200 dark:divide-gray-700">
                    {!posts.length && "No posts found."}
                    {posts.map((post) => {
                        const { slug, title, date, summary, tags } = post;
                        return (
                            <li key={slug} className="py-12">
                                <article>
                                    <div
                                        className="space-y-2 xl:grid xl:grid-cols-4 xl:items-baseline xl:space-y-0">
                                        <dl>
                                            <dt className="sr-only">Published on</dt>
                                            <dd className="text-base font-medium leading-6 text-gray-500 dark:text-gray-400">
                                                <time dateTime={date}>{date}</time>
                                            </dd>
                                        </dl>
                                        <div className="space-y-5 xl:col-span-3">
                                            <div className="space-y-6">
                                                <div>
                                                    <h2 className="text-2xl font-bold leading-8 tracking-tight">
                                                        <Link
                                                            href={`/blog/${slug}`}
                                                            className="text-gray-900 dark:text-gray-100"
                                                        >
                                                            {title}
                                                        </Link>
                                                    </h2>
                                                    <div className="flex flex-wrap">
                                                        {tags.map((tag) => (
                                                            <Tag key={tag} text={tag} />
                                                        ))}
                                                    </div>
                                                </div>
                                                <div className="prose max-w-none text-gray-500 dark:text-gray-400">
                                                    {summary}
                                                </div>
                                            </div>
                                            <div className="text-base font-medium leading-6">
                                                <Link
                                                    href={`/blog/${slug}`}
                                                    className="text-primary-500 hover:text-primary-600 dark:hover:text-primary-400"
                                                    aria-label={`Read more: "${title}"`}
                                                >
                                                    Read more &rarr;
                                                </Link>
                                            </div>
                                        </div>
                                    </div>
                                </article>
                            </li>
                        );
                    })}
                </ul>
                {total > MAX_POSTS && (
                    <div className="flex justify-end text-base font-medium leading-6">
                        <Link
                            href="/blog"
                            className="text-primary-500 hover:text-primary-600 dark:hover:text-primary-400"
                            aria-label="All posts"
                        >
                            All Posts &rarr;
                        </Link>
                    </div>
                )}
            </div>
        </>
    );
}
