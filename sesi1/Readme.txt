- Software Testing > menguji aplikasi atau software yang lagi dibuat oleh developer dan juga menilai sistem dan mencari error atau bugs sehingga pengguna sebenarnya nanti yang menggunakan sudah bagus.

Yang melakukan pengujian :
> Semua anggota tim turut menguji, terlepas dev, product owner, tester dan lead/manager.

- product Owner 	: menginterview, ke Manager atau User
- Developer 	: UnitTest
- Tester		: Program berjalan secara fungsional, usability, ui experience, 			  security, dll.
- Lead/manager/Architectur : Mengawal atau merencanakan strategi rencana pengujian


Kapan seharusnya pengujian dimulai?
- Perancangan Requirement	> Membuat acceptence criteria(kondisi ketika kriteria2 apa yg bisa di anggap mencukupi, menentukan definisi selesai, cth: membuat requirement register: Melakukan pendaftaran, input username, pass, no hp, di submit kemudian digunakan utk login).

- Spesifikasi Fungsional	> Membuat skenario fungsional
- Implementasi			> Membuat otomatisasi pengujian
- Kode selesai			> Menjalankan rencana pengujian

Bagaimana jika dikatakan pengujian telah selesai?

-Manual > mengatur strategi dan rencana pengujian, persiapan data, regression test(cth pd ecommerce: test dr awal/home page, login register, melakukan pencarian, pembelian, melakukan pembayaran, metode pembayaran, review, test batal, dll) dan eksekusi pengujian.

- Otomatisasi > unit test, integration test (cth: test integrasi antar suatu modul aplikasi, melakukan interaksi dr mobile, mengirimkan data ke back end, melakukan proses), UI Test(test tampilan, cek typo, ui experience), performence test (performa aplikasi), security test dan tergantung kebutuhan.

QA > peran nya utk memastikan kualitas dari sebuah produk, sama hal nya di dalam software developmentm utk memastikan kualitas tsb, maka dilakulan proses proses yang menghasilkan output seperti dokumentasi, laporan dan atau metrik kualitas dari sebuah produk.

Apa itu QA > Software Tester, but software tester belum tentu QA


Jenis Testing

1. Black Box Testing > functional testing (tdk perlu tahu struktur internal kode)
	Input > Software Under Test (Black Box) > Output 
	Lampiran Bug Report : Screenshoot, screen bug layar.

Sedini mungkin yang dilakukan utk QA Test :
> Bikin Test Case dasar : dr dokument requirement, dr design tampilan mock up.
Saran : Bikin tase case yg dasar-dasar dahulu, seperti fungsi dasar nya, jika sudah baru coba test Negative flow atau negatif case.


2. White Box Testing > Functional Testing (udh terlihat program)
	Cara pengerjaan: cek detail alur rancangan (flow chart/diagram)

Metodelogi Testing Development:
Scrum > request dr stake holder, user ke product owner
"Water development"

Integration testing
> End-to-end : testing dr awal sampai akhir, memastikan dr awal input login atau homepage sampai ke db itu udh sesuai atau tidak dengan requirement nya.
> Dapat dikombinasikan dengan exploratory test.

System test
> cakupan yang cukup besar dan dilakukan ketika integration test sudah selesai. Pengujian dilakukan secara keseluruhan terhadap sebuah software. Tidak boleh ada error atau impact dr proses nya.

Salah satu pengujian system test adalah regression test.
> pengujian software memastikan bahwa penambahan atau perubahan code baru tidak ada mempengaruhi code yang sudah ada.

Panduan:
1. Dilakukan dengan mengeksekusi semua test case yang ada, mencakup software secara keseluruhan.
2.mengeksekusi sebagian test case yang berkaitan dengan penambahan fitur.
3. mengeksekusi semua test case yang mempunyai prioritas/level tertentu.

Acceptance Test
> Acceptance test adalah tahap uji terakhir pada functional testing level. Tahap ini QA secara umum tidakterlibat langsung, karena yang bertanggung jawab pada tahap acceptancetest adalah product manager atau tim yang berhubungan langsung dengan user yang akan menggunakan software.
> Hal yang perlu diperhatikan dalam acceptance test ini yaitu software yang akan diuji sudah melewati tahapujisebelumnya, kemudian mempersiapkan dokumentasi serta alur test yang akan dieksekusi menyesuaikandengan user story dan alur bisnis.
> Ketika sedang melaksakan acceptance test, behavior user dalam menggunakan software juga perludiperhatikan. Tujuannya adalah untuk mengevaluasi apakah fitur tersebut sudah nyaman digunakan ataumasih butuh improvisasi.

Tinggal bagaimana QA tersebut dapat mengedukasi dirinya sendiri dan tim dalam mengembangkan danmemastikan kualitas dari sebuah software.

Teknik Testing
ada 2, manual dan otomatis
1. Manual Testing
	Langkah untuk mencari cacat atau bug pada program perangkat lunak, pada metode ini tester/penguji memiliki peran penting sebagai pengguna akhir untuk pengecekan semua fituraplikasi bekerja dengan benar sehingga bisa memiliki SKEMA TESTING.
Apa yang diharapkan dari SKEMA TESTING ? mempermudah Penguji/Tester untuk melakukan automation testing baik dari penyiapan sisi script dan urutantesting serta membagi testing dalam format POSITIF & NEGATIF TESTING.

2. Analisa
	> Jangan mengekspos data mana yg valid dan yg tidak valid. jika data sudah ada di sistem.


Automatiton Testing
Menemukan bug secara berkala

Kapan dilakukan manual test dan automation test?
Manual ketika utk request requirment
Automation ketika dilakukan untuk menemukan bug coding secara berkala.Atau jika melakukan test secara banyak.

Mindset QA
1. Paham alur perubahan perusahaan
2. Belajar hal yang kecil
3. Belajar
4. Belajar coding
5. Buat developer menjadi partner
6. Join friends network
7. Mengambil kesempatan
8. Decide the project structure