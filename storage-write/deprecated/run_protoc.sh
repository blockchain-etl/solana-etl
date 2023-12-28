mkdir -p pbcodegen
cd protosrc
for file in *.proto;
	do protoc --go_out=../pbcodegen --go_opt=paths=source_relative --go-grpc_out=../pbcodegen --go-grpc_opt=paths=source_relative $file; 
done
