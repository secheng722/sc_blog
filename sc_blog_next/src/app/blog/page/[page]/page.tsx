'use client'
import ListLayOut from "@/app/components/list_layout_tags";
import {useEffect, useState} from "react";
import {post} from "@/app/main";

const POSTS_PER_PAGE = 5

export interface PaginationProps {
    totalPages: number
    currentPage: number
}

export default function Page({ params }: { params: { page: string } }) {

    const title:string = "Latest";


    const [posts, setPosts] = useState<post[]>([]);

    const [pagination, setPagination] = useState<PaginationProps>()



    useEffect(() => {
        const pageNumber = parseInt(params.page as string)
        const fetchTotals = async () => {
            const res = await fetch("/api/post/get_ports_all_count");
            const data = await res.json();
            if (data.code === 200) {
                setPagination({
                    currentPage: pageNumber,
                    totalPages: Math.ceil(data.data / POSTS_PER_PAGE),
                })
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
                            page: pageNumber,
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
