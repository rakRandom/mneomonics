import { redirect } from '@sveltejs/kit';
 
export function load() {
    // TODO!: Add credentials verification
    // Note: The credentials verification should be a prerequisite to load any page, except '/login'
    redirect(302, '/login');
}
