import Link from "next/link"

interface Props {
  text: string,
}

const Tag = ({ text }: Props) => {
  return (
    <Link href={`/tags/${text}`}
      className="mr-3 text-sm font-medium uppercase text-pink-500 hover:text-pink-300 "
    >
      {text.split(' ').join('-')}
    </Link>
  )
}

export default Tag;
