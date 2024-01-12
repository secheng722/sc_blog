'use client'

import { useState } from "react"
import { requestUrl } from "./prefixCommon"

type loginUser = {
  username: string,
  password: string
}

type registerUser = {
  username: string,
  password: string,
  email: string
}

const login = async (user: loginUser) => {
  console.log('login', user)
  try {
    const response = await fetch('/api/user/login', {
      method: 'post',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(user)
    })
    if (response.ok) {
      const res = await response.json()
      console.log(res)
    } else {
      console.log(response.statusText)
    }
  }
  catch (err) {
    console.log(err)
  }
}

export default function Form({ type }: { type: string }) {
  const [user, setUser] = useState<loginUser | registerUser>({
    username: '',
    password: '',
  })
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setUser({
      ...user,
      [name]: value
    })
  }


  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    login(user)
  }

  return (
    <div className="w-72">
      {/* 如果是注册页面，就显示下面的内容 */}
      <h1 className="text-xl font-semibold">Welcome back</h1>
      <small className="text-gray-500">Please  your account</small>
      <form className="mt-4" onSubmit={handleSubmit}>
        <div className="mb-3">
          <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="username">
            Username
          </label>
          <input
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            name="username"
            type="text"
            placeholder="Username"
            value={user.username}
            onChange={handleChange}
          />
        </div>
        <div className="mb-3">
          <label className="block text-gray-700 text-sm font-bold mb-2" htmlFor="password">
            Password
          </label>
          <input
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            name="password"
            type="password"
            placeholder="Password"
            value={user.password}
            onChange={handleChange}
          />
        </div>
        <div className="mb-3 flex content-center">
          <input className="mr-1 checked:bg-purple-700" type="checkbox" id="remember" />
          <label className="mr-auto text-xs font-semibold">Remember me</label>
          <a className="ml-auto text-xs font-semibold text-purple-700" href="#">
            Forgot Password?
          </a>
        </div>
        <div className="mb-3">
          <button className="bg-purple-700 hover:bg-purple-900 text-white font-bold py-2 px-4 w-full rounded focus:outline-none focus:shadow-outline">
            Login
          </button>
        </div>
        <div className="text-center">
          <span className="text-xs text-gray-400 font-semibold">Don't have account?</span>
          <a className="text-xs text-purple-700 font-semibold" href="#"> Sign up</a>
        </div>
      </form>
    </div>
  )
}
