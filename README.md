# Reflection Module-8

### 1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable? 

Unary itu paling sederhana: satu request, satu response, cocok buat operasi ringan seperti login atau submit data. Server streaming dipakai kalau datanya banyak dan dikirim bertahap, misalnya riwayat transaksi. Kalau bidirectional streaming, dua-duanya bisa kirim data terus-menerus, biasanya dipakai di chat atau aplikasi real-time.

### 2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?

Untuk keamanan di gRPC, hal utama yang harus diperhatikan itu autentikasi (siapa yang akses), otorisasi (boleh ngapain), dan enkripsi (data aman di perjalanan). Biasanya pakai TLS buat enkripsi, dan bisa tambah token seperti JWT untuk autentikasi. Kalau ini diabaikan, data bisa bocor atau disalahgunakan.

### 3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?

Bidirectional streaming itu fleksibel tapi juga ribet. Masalah yang sering muncul misalnya sinkronisasi data, handling error di tengah stream, atau koneksi tiba-tiba putus. Di aplikasi chat, ini bisa bikin pesan telat, dobel, atau bahkan hilang kalau nggak ditangani dengan benar.

### 4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?

ReceiverStream enaknya gampang dipakai dan cocok buat integrasi dengan Tokio, jadi streaming bisa jalan async dengan rapi. Tapi kekurangannya, kita harus ngatur channel sendiri (buffer, error, dll), jadi kalau salah konfigurasi bisa bikin performa jelek atau malah blocking.

### 5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?

Supaya kode lebih rapi dan gampang dikembangin, biasanya dipisah per layer: service, handler, dan model. Bisa juga pakai module terpisah untuk tiap fitur. Selain itu, pakai trait dan abstraction biar logic bisa dipakai ulang tanpa harus nulis ulang dari awal.

### 6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?

Untuk payment yang lebih kompleks, nggak cukup cuma return sukses. Perlu validasi input, cek saldo, integrasi ke sistem lain (misalnya bank), logging, dan error handling yang jelas. Kadang juga perlu retry mechanism kalau ada kegagalan.

### 7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?

Pakai gRPC biasanya bikin sistem lebih cepat dan efisien, terutama di arsitektur microservices. Tapi di sisi lain, integrasi dengan sistem lain bisa lebih tricky, apalagi kalau mereka belum pakai gRPC. Jadi perlu dipikirin soal kompatibilitas juga.

### 8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?

HTTP/2 itu lebih cepat karena bisa kirim banyak request dalam satu koneksi (multiplexing), dan ada fitur seperti header compression. Dibanding HTTP/1.1, jelas lebih efisien. Tapi implementasinya lebih kompleks, dan nggak semua tools lama support dengan baik.

### 9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?

REST itu modelnya request-response biasa, jadi kurang cocok buat komunikasi real-time. Kalau butuh update terus-menerus, harus polling atau pakai WebSocket. Sementara gRPC streaming bisa langsung kirim data terus tanpa nunggu request baru, jadi lebih responsif.

### 10.What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?

Protobuf di gRPC itu ketat karena harus sesuai schema, jadi lebih aman dan konsisten, tapi kurang fleksibel kalau sering berubah. JSON di REST lebih bebas dan gampang diubah, tapi rawan error kalau struktur datanya nggak konsisten. Jadi ada trade-off antara fleksibilitas dan ketertiban.
