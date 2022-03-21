#include <stdio.h>
#include <stdint.h>

int main()
{
        uint32_t v[] = {0x00010203, 0x04050607}, k[] = {0, 0, 0, 0};
  
	
		// uint32_t v[] = {0xLJBLKECB, 0x0CELCIMF};
		// 64 bit ASCII->HEX translation of LJBLKECB: 4C4A424C4B454342  
		// 64 bit ASCII->HEX trnaslation of 0CELCIMF: 3043454C43494D46
		
		printf(" Original Values: ");
		printf("[ %X %X ]", v[0], v[1]);
		set_key(k);
        encrypt(v);

        printf("\n Encrypted:       ");
        printf("[ %X %X ]", v[0], v[1]);

        decrypt(v);
        printf("\n Decrypted:       ");
        printf("[ %X %X ]", v[0], v[1]);
        printf("\n");

        return 0;
}