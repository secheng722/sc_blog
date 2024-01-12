import Form from "@/components/form";

const type = "Login"

export default function Login() {

  return (
    <div className="flex h-screen bg-gray-200">
      <div className="w-full max-w-2xl  m-auto bg-white rounded p-5">
        <div className="mb-4">
          {/* 左边是图片 右边是form */}
          <div className="flex justify-center">
            <div className="w-1/2">
              <img src="https://picsum.photos/200/300" />
            </div>
            <div className="w-1/2">
              <Form type={type}></Form>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
