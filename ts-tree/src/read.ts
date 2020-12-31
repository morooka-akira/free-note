import fs, { Dirent } from 'fs'
import path from 'path'

export type Node = {
    type: 'file' | 'directory'
    name: string
    children: Node[]
}

const readDirectory = (dir: string): Node[] => {
    const dirents = fs.readdirSync(dir, {
        withFileTypes: true
    })

    let nodes: Node[] = []
    dirents.forEach((dirent)  => {
        if (dirent.name.startsWith('.')) {
            return
        }
        if (dirent.isFile()) {
            nodes.push({
                type: 'file',
                name: dirent.name,
                children: []
            })
        } else if (dirent.isDirectory()) {
            nodes.push({
                type: 'directory',
                name: dirent.name,
                children: readDirectory(
                    path.join(dir, dirent.name)
                )
            })
        }
    })
    return nodes
}

export const read = (dir: string): Node => {
    let stat
    try {
        stat = fs.statSync(dir)
    } catch (e) {
        throw new Error(`"${dir}" does not exist.`)
    }

    if (!stat.isDirectory()) {
        throw new Error(`"${dir}" can't be opened as a directory.`)
    }

    const root: Node = {
        type: 'directory',
        name: dir,
        children: readDirectory(dir)
    }

    return root
}
