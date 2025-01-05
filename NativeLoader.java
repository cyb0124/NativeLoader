package cyb0124;

import org.apache.commons.io.IOUtils;
import org.apache.logging.log4j.LogManager;

import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;

public class NativeLoader {
    public static String ARCH;
    public static boolean IS_WINDOWS;

    public native static long allocPagesRW(long size);

    public native static ByteBuffer wrapBuffer(long base, long size);

    public native static void setAndRunPagesRX(long base, long rxSize, long table, long entry, Object arg);

    static {
        ARCH = System.getProperty("os.arch").toLowerCase().trim();
        if (ARCH.equals("x86_64") || ARCH.equals("amd64")) {
            ARCH = "x64";
        } else if (!ARCH.equals("aarch64")) {
            throw new UnsupportedOperationException("Unsupported architecture: " + ARCH);
        }
        String srcPath = "/NativeLoader-" + ARCH;
        String os = System.getProperty("os.name");
        if (os.startsWith("Linux")) {
            srcPath += ".so";
        } else if (os.startsWith("Windows")) {
            srcPath += ".dll";
            IS_WINDOWS = true;
        } else if (os.startsWith("Mac") || os.startsWith("Darwin")) {
            srcPath += ".dylib";
        } else {
            throw new UnsupportedOperationException("Unsupported OS: " + os);
        }
        Path dstPath = Paths.get("natives" + srcPath).toAbsolutePath();
        try (InputStream is = NativeLoader.class.getResourceAsStream(srcPath)) {
            Files.createDirectories(Paths.get("natives"));
            Files.write(dstPath, IOUtils.toByteArray(is), StandardOpenOption.WRITE, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
        } catch (IOException e) {
            LogManager.getLogger().error("Failed to extract native binary", e);
        }
        System.load(dstPath.toString());
    }
}
