```bash
ffmpeg -i input.mp3 -c:a aac -b:a 128k -vn -hls_time 10 -hls_list_size 0 -hls_segment_filename "output%03d.ts" output.m3u8
```
