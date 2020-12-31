import { read } from './read'
import { format } from './format'
import meow from 'meow'

type Writer = (...args: any[]) => void

export const main = (argv: string[], stdout: Writer, stderr: Writer) => {
    const cli = meow(
        `
        Usage 
            $ ts-tree <directory>

        Examples 
            $ ts-tree 
            $ ts-tree path/to/dir
        `,
        {
            argv
        }
    )

    const dir = cli.input[0] || '.'

    let root

    try {
        root = read(dir)
    } catch (e) {
        stderr(`Error: ${e.message}`) 
        return 1
    }
    const output = format(root)

    stdout(dir)

    return 0
}