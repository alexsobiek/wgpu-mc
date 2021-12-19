package dev.birb.wgpu.rust;

//import net.minecraft.util.Identifier;

//import net.minecraft.world.chunk.ChunkSection;

import net.minecraft.world.chunk.ChunkSection;
import net.minecraft.world.chunk.WorldChunk;
import org.jetbrains.annotations.Nullable;

import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;
import java.util.HashMap;

public class WgpuNative {

    public static native int getTextureId(String identifier);

    public static native void initialize(String title);

    public static native void initRenderer();

    public static native void updateWindowTitle(String title);

    public static native void registerEntry(int type, String name);

    public static native void doEventLoop();

    public static native void uploadChunk(WorldChunk chunk);

    public static native void uploadChunkSimple(int[] blocks, int x, int z);

    public static native byte[] digestInputStream(InputStream stream);

    public static native String getBackend();

    public static native HashMap<String, Integer> bakeBlockModels();

    public static native void setWorldRenderState(boolean render);

    public static native void bindBuffer(int target, int buffer);

    public static native int uploadBuffer(long ptr, long length, int usage);

    public static native int genBuffer();

    public static native void deleteBuffer(int buffer);

    public static native void drawArray(int mode, int first, int count);

    public static native void vertexPointer(int size, int type, int stride, long pointer);

    public static native void colorPointer(int size, int type, int stride, long pointer);

    public static native void matrix(long floatBuffer);

    public static native void pushMatrix();

    public static native void popMatrix();

    public static native int genTexture();

    public static native void texImage2D(int target, int level, int internalFormat, int width, int height, int border, int format, int type, long pixels_ptr);

    public static native void activeTexture(int slot);

    public static native void bindTexture(int textureId);

    public static native void submitCommands();

    public static native int getMaxTextureSize();

    public static native int getWindowWidth();

    public static native int getWindowHeight();

    public static native void texCoordPointer(int size, int type, int stride, long pointer);

    public static native void enableClientState(int cap);
}
