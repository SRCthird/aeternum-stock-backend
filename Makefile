driver: src/driver.o src/app/app.controller.o src/api/user/user.controller.o src/core/environment.o
	#g++ src/driver.o src/app/app.controller.o src/core/environment.o -I/usr/include/mysql/ -o CrowsNest -lpthread -lssl -lcrypto  -lsoci_core -lsoci_mysql -lmysqlclient -DCROW_ENABLE_SSL
	g++ src/driver.o src/app/app.controller.o src/api/user/user.controller.o src/core/environment.o -I/usr/include/mysql/ -o CrowsNest -lpthread -lssl -lcrypto -lsoci_core -lsoci_sqlite3 -lsqlite3 -DCROW_ENABLE_SSL

src/driver.o: src/driver.cpp src/core/routeManager.h
	g++ -c src/driver.cpp -o src/driver.o -DCROW_ENABLE_SSL -I/usr/include/mysql/

src/driver.cpp:
	$(error driver.cpp not found in /src/core)

src/app/app.controller.o: src/app/app.controller.cpp src/app/app.service.h
	g++ -c src/app/app.controller.cpp -o src/app/app.controller.o -DCROW_ENABLE_SSL -I/usr/include/mysql/

src/app/app.service.h:
	$(error app.service.h not found in /src/app)

src/api/user/user.controller.o: src/api/user/user.controller.cpp src/api/user/user.service.h
	g++ -c src/api/user/user.controller.cpp -o src/api/user/user.controller.o -DCROW_ENABLE_SSL -I/usr/include/mysql/

src/api/user/user.service.h:
	$(error user.service.h not found in /src/api/user)

src/api/user/user.controller.cpp:
	$(error user.controller.cpp not found in /src/api/user)

src/core/environment.o: src/core/environment.cpp
	g++ -c src/core/environment.cpp -o src/core/environment.o 

src/core/environment.cpp:
	$(error environment.cpp not found in src/core)

test:
	cd src/api/user && source ./test.sh
	cd src/app && source ./test.sh

clean:
	rm -f src/driver.o src/app/*.o src/api/user/*.o src/core/*.o

run:
	sudo ./CrowsNest
