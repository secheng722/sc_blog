'use client'
import ListLayOut from "@/app/components/list_layout_tags";
import {post} from "@/app/main";
import {useEffect, useState} from "react";

const POSTS_PER_PAGE = 5;


export default function Blog() {

    const [posts, setPosts] = useState<post[]>([]);

    const [pageNumber, setPageNumber] = useState(1);

    const [totalPages, setTotalPages] = useState(1);

    const title:string = "Latest";

    const pagination = {
        currentPage: pageNumber,
        totalPages: Math.ceil(totalPages / POSTS_PER_PAGE),
    }

    useEffect(() => {
        const fetchTotals = async () => {
            const res = await fetch("/api/post/get_ports_all_count");
            const data = await res.json();
            if (data.code === 200) {
                setTotalPages(data.data);
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
                            page_size: POSTS_PER_PAGE,
                            tag: "",
                        }),
                    }
                );
                const data = await res.json();
                if (data.code === 200) {
                    setPosts(data.data);
                }
            }
        ;
        fetchTotals().then(r => fetchPosts());
    }, []);


    return (
        <ListLayOut
            title={title}
            displayPosts={posts}
            pagination={pagination}
        ></ListLayOut>
    )
}
