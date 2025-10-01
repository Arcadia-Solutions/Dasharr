export const bytesToReadable = (bytes: number): string => {
  const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB']
  let size = bytes
  let unitIndex = 0

  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }

  return `${size.toFixed(unitIndex === 0 ? 0 : 2)} ${units[unitIndex]}`
}
export const timeAgo = (date: string | null) => {
  if (date === null) {
    return 'Never'
  }
  const diff = (Date.now() - new Date(date).getTime()) / 1000
  return diff < 60
    ? `${Math.floor(diff)}s ago`
    : diff < 3600
      ? `${Math.floor(diff / 60)}m ago`
      : diff < 86400
        ? `${Math.floor(diff / 3600)}h ago`
        : `${Math.floor(diff / 86400)}d ago`
}
