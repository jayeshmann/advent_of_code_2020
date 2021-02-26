/**
 * Copyright (C) 2021 Pooja Tanwar
 */


import java.io.*;
import java.util.*;


class Pooja {

	public static void main(String args[]) {
		try {
			int a[] = new int[10];
			File f1 = new File("a.in");
			Scanner sc = new Scanner(f1);

			int j = 0;
			while (sc.hasNextLine()) {

				a[j] = (int) sc.nextInt();
				System.out.println(a[j]);

				j++;
			}
			System.out.println(a[2]);
			for (int i = 0; i < a.length; i++) {
				for (int k = 1; k < a.length; k++) {
					if (a[i] + a[k] == 2020) {
						System.out.println(a[i] * a[k]);
					}
				}
			}
			sc.close();
		} catch (Exception e) {
			System.out.println(e);
		}
	}
}
