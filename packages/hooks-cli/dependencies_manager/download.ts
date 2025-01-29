interface DownloadFileOptions {
  url: string; // URL of the remote file
  targetDir: string; // Directory where the file will be saved
  fileName: string; // The name to save the file as (including extension)
}

/**
 * const options: DownloadFileOptions = {
  url: "https://example.com/path/to/large-file.zip", // Replace with actual file URL
  targetDir: "./downloads", // Ensure this is a valid directory
  fileName: "large-file.zip", // Name of the file to save
};

await downloadFile(options);
 */
export async function downloadFile(
  { url, targetDir, fileName }: DownloadFileOptions,
) {
  // Check if the target directory exists
  try {
    const dirInfo = await Deno.stat(targetDir);
    if (!dirInfo.isDirectory) {
      throw new Error(`${targetDir} is not a valid directory.`);
    }
  } catch (err) {
    if (err instanceof Error) {
      console.error(`Directory not found: ${err.message}`);
    } else {
      console.error(`Directory not found: ${String(err)}`);
    }
    return;
  }

  const filePath = `${targetDir}/${fileName}`;
  const response = await fetch(url);

  if (!response.ok || !response.body) {
    throw new Error(`Failed to download file: ${response.statusText}`);
  }

  const file = await Deno.open(filePath, {
    write: true,
    create: true,
    truncate: true,
  });

  try {
    // Stream the response body to the file
    await response.body.pipeTo(file.writable);
  } catch (error) {
    console.error("Error writing file:", error);
  }
}
