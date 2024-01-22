import SocialIcon from "@/app/components/social-icons";


export function Footer() {
  return (
      <footer>
          <div className="mt-16 flex flex-col items-center">
              <div className="mb-3 flex space-x-4">
                  <SocialIcon kind="github" href={`https://github.com/secheng722`} size={6} />
              </div>
              <div className="mb-2 flex space-x-2 text-sm text-gray-500 dark:text-gray-400">
                  <div>{`secheng722`}</div>
                  <div>{` • `}</div>
                  <div>{`© ${new Date().getFullYear()}`}</div>
                  <div>{` • `}</div>
              </div>
              <div className="mb-8 text-sm text-gray-500 dark:text-gray-400">
              </div>
          </div>
      </footer>

  )};
