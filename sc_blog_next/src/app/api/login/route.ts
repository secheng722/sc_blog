import { NextRequest, NextResponse } from "next/server";

//post login方法
//
//
export async function POST(req: NextRequest,) {
  return NextResponse.json({ code: 200, message: "登录成功" });
}
