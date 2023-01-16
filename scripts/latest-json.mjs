import {exec} from 'node:child_process'
import {readFile, writeFile} from 'node:fs/promises'
import {join} from 'node:path'
import glob from 'tiny-glob'

const BASE_URL = 'https://github.com/elk-zone/elk-native/releases/download'

async function main() {
  process.chdir(join(process.cwd(), './artifacts'))

  const tag = process.argv[2]
  const channel = process.argv[2] // either nightly or stable

  const obj = {
    name: tag.replace('v', ''),
    notes: await getNotesForTag(tag),
    pub_date: new Date().toISOString(),
    platforms: {
      'darwin-aarch64': await getPlatform(tag, 'aarch64', [
        'app.tar.gz',
        'app.tar.gz.sig'
      ]),
      'darwin-x86_64': await getPlatform(tag, 'x86_64', [
        'app.tar.gz',
        'app.tar.gz.sig'
      ]),
      'linux-x86_64': await getPlatform(tag, 'x86_64', [
        'AppImage.tar.gz',
        'AppImage.tar.gz.sig'
      ]),
      'windows-x86_64': await getPlatform(tag, 'x86_64', ['msi.zip', 'msi.zip.sig'])
    }
  }

  writeFile(`latest-${channel}.json`, JSON.stringify(obj, null, 2))
}
main()

async function getNotesForTag(tag) {
  // echo "$(git tag -l --format='%(contents:subject)%0a%(contents:body)' desktop/0.0.3)"
  return execCmd('git', [
    'tag',
    '-l',
    '--format="%(contents:subject)%(contents:body)"',
    tag
  ])
}

async function getPlatform(tag, arch, exts) {
  const [artifact, signature] = await glob(`**/*${arch}.{${exts.join(',')}}`)

  return {
    url: `${BASE_URL}/${tag}/${artifact}`,
    signature: await readFile(signature, 'utf-8')
  }
}

async function execCmd(cmd, args, options = {}) {
  return new Promise((resolve, reject) => {
    exec(
      `${cmd} ${args.join(' ')}`,
      {...options, encoding: 'utf-8'},
      (error, stdout, stderr) => {
        if (error) {
          console.error(
            `Failed to execute cmd ${cmd} with args: ${args.join(
              ' '
            )}. reason: ${error}`
          )
          reject(stderr)
        } else {
          resolve(stdout)
        }
      }
    )
  })
}
