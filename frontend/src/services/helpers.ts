// export const bytesToReadable = (bytes: number): string => {
//   const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB']
//   let size = bytes
//   let unitIndex = 0

//   while (size >= 1024 && unitIndex < units.length - 1) {
//     size /= 1024
//     unitIndex++
//   }

//   return `${size.toFixed(unitIndex === 0 ? 0 : 2)} ${units[unitIndex]}`
// }
