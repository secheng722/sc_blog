
import headerNavLinks from "@/app/components/nav_links";
import Link from "next/link";
const Header = () => {
    return (
        <div className="flex items-center justify-between py-10  ">
            <div className="flex-1">
                <Link href="/" className=" text-xl">SC_BLOG</Link>
            </div>
            <div className="flex-none">
                {headerNavLinks.map((link) => (
                    <Link href={link.href} key={link.title}
                         className="px-1 text-pink-500 hover:text-pink-300 ">{link.title}
                    </Link>
                ))}
            </div>
        </div>
    );
}

export default Header;
