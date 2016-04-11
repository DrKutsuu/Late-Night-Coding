/** A bit of random code to learn about trigonometrics and space physics
 * 
 */

#include <stdio.h>
#include <math.h>

/** Calculates the ballistic velocity for a craft escaping orbit (towards the planet)
 *
 */ 
void suborbital(double altitude, double speed) {
	float g = 0.096; //km/s increase in speed from gravity
	
	
}

/** Calculates the total velocity of a craft on a ballistic path
 */ 
void ballisticTrajectory(double lX, double lY, double dX, double dY) {
	
}

/** Radians can be calculated using x*Pi/180 where x is the number in degrees to be converted
 */ 
double degreesToRadians(double degrees) {
	return degrees*M_PI/180;
}

/** Calculates great-circle distance between two given x,y points on a sphere
 * 1. haversin (x) = sin^2(x/2)=(1-cos(x))/2
 * 2. haversin (distance/radius of sphere) = haversin(lat2-lat1) + (cos(lat1)cos(lat2) haversin(long2-long1))
 */ 
double haversin(double x) {
	return (1-cos(x))/2;
}
 
void haversine(double* distance, double* radius, double lat1, double long1, double lat2, double long2) {
	/*haversin (distance/radius of sphere) = 
	haversin(lat2-lat1) + 
	(cos(lat1)cos(lat2) haversin(long2-long1))
	
	double distX, distY, distZ;
	
	long1 -= long2;
	long1 = degreesToRadians(long1);
	
	lat1 = degreesToRadians(lat1);
	lat2 = degreesToRadians(lat2);
	
	distZ = sin(lat1) - sin(th2);
	distX = cos(long1)* cos(lat1) - cos(lat2);
	distY = sin(long1)* cos(lat1);
	*/
	
}



main() {
	double lX;
	double lY;
	double dX;
	double dY;
	
	//ballisticTrajectory()
	
	
}
