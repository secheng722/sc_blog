'use client'
import { useEffect, useState } from "react"
import Markdown from "react-markdown"
import remarkGfm from "remark-gfm"



const markdown = ` # This is a *header*
A paragraph with *emphasis* and **strong importance**.

> A block quote with ~strikethrough~ and a URL: https://reactjs.org.

* Lists
* [ ] todo
* [x] done

A table:

| a | b |
| - | - |
`
type Article = {
  id: number,
  title: string,
  content: string,
  intro: string,

}



export default function Blog() {
  //页面加载时，获取文章列表
  const [articles, setArticles] = useState<Article[]>([])

  useEffect(() => {
    const getArticles = async () => {
      try {
        const response = await fetch('/api/blog/getlist', {
          method: 'get',
          headers: {
            'Content-Type': 'application/json'
          }
        })
        if (response.ok) {
          const res = await response.json()
          console.log(res.data)
          setArticles(res.data)
        } else {
          console.log(response.statusText)
        }
      }
      catch (err) {
        console.log(err)
      }
    }
    getArticles()
  })
  return (
    <section className="bg-white dark:bg-gray-900">
      <div className="container px-6 py-10 mx-auto">
        <h1 className="text-3xl font-semibold text-gray-800 capitalize lg:text-4xl dark:text-white">
          From the blog
        </h1>
        <div className="grid grid-cols-1 gap-8 mt-8 md:mt-16 md:grid-cols-2">
          {/* 文章列表 */}
          {articles.map((item, index) => (
            <div className="lg:flex" key={index}>
              <img className="object-cover w-90 h-56 rounded-md lg:h-64" src="https://picsum.photos/400/300" alt="Article" />
              <div className="flex flex-col justify-between py-6 lg:mx-6">
                <a href="#" className="text-xl font-semibold text-gray-800 hover:underline dark:text-white">
                  {item.title}
                </a>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )

} 
