package com.jojonomic.katalon.web


import java.util.regex.Matcher
import java.util.regex.Pattern

import org.openqa.selenium.By
import org.openqa.selenium.JavascriptExecutor
import org.openqa.selenium.WebElement
import org.openqa.selenium.support.ui.ExpectedConditions

import com.kms.katalon.core.annotation.Keyword


public class HelperVue extends Common {

	@Keyword
	public static checkboxSelect(int totalCheckbox){
		wait.until(ExpectedConditions.visibilityOfElementLocated(By.xpath("//div[contains(@class,'vue-treeselect__option')]/div//label")))
		List<WebElement> testData = driver.findElements(By.xpath("//div[contains(@class,'vue-treeselect__option')]/div//label"))
		loopingData : for(int i=1; i <= testData.size(); i++){
			testData.get((i-1)).click()
			if(i >= totalCheckbox){
				break loopingData;
			}
		}
	}

	@Keyword
	public static ArrayList<String> checkboxSelectReturn(int totalCheckbox){
		JavascriptExecutor js =(JavascriptExecutor)driver;

		ArrayList<String> email = new ArrayList<String>();
		ArrayList<String> getUser = new ArrayList<String>();
		wait.until(ExpectedConditions.visibilityOfElementLocated(By.xpath("//div[contains(@class,'vue-treeselect__option')]/div//label")))
		List<WebElement> testData = driver.findElements(By.xpath("//div[contains(@class,'vue-treeselect__option')]/div//label"))
		loopingData : for(int i=1; i <= testData.size(); i++){
			testData.get((i-1)).click()
			email.add(testData.get((i-1)).getText())
			if(i >= totalCheckbox){
				break loopingData;
			}
		}
		println(email.toString())
		Pattern p = Pattern.compile("(?<=\\()(.*?)(?=\\))")
		Matcher m = p.matcher(email.toString())

		while(m.find()){
			println(m.group(1))
			getUser.add(m.group(1))
		}
		println(getUser)
		return getUser;
	}

	@Keyword

	public static CheckBoxDelete(int totalData){
		List<WebElement> DataDelete = driver.findElements(By.xpath("//div[@class='custom-control custom-checkbox custom-control-inline']"))
		println(DataDelete.size())
		loopingData : for(int i=1; i < DataDelete.size(); i++){
			DataDelete.get((i-1)).click()
			if(i >= totalData){
				break loopingData;
			}
		}
	}

	@Keyword
	public static ClickHiglight(String inputDropdown, int ClickDropdown){
		List<WebElement> ClickRule = driver.findElements(By.xpath('//input[@class="vue-treeselect__input"]'))
		println(ClickRule.size())
		ClickRule.get(ClickDropdown-1).click()
		ClickRule.get(ClickDropdown-1).sendKeys(inputDropdown)

		wait.until(ExpectedConditions.visibilityOfElementLocated(By.xpath("//div[contains(@class,'vue-treeselect__option--highlight') or contains(@class,'vue-treeselect__option--matched')]/div//label")))

		WebElement InputRule = driver.findElement(By.xpath("//div[contains(@class,'vue-treeselect__option--highlight') or contains(@class,'vue-treeselect__option--matched')]/div//label"))

		InputRule.click()
	}

	@Keyword
	public static SingleSelect(String SelectVar, int ClickDropdown){
		List<WebElement> ClickRule = driver.findElements(By.xpath('//input[@class="vue-treeselect__input"]'))
		println(ClickRule.size())
		ClickRule.get(ClickDropdown-1).click()

		WebElement InputRule = driver.findElement(By.xpath("//div[contains(@class,'vue-treeselect__option')]/div//label//b[contains(text(),'"+SelectVar+"')]"))

		InputRule.click()
	}

	@Keyword
	public static DataUpdatePolicy(String dataUpdate){
		WebElement SelectDataUpdate = driver.findElement(By.xpath('//div[contains(text(),"'+ dataUpdate+'")]'))
		SelectDataUpdate.click()
	}

	@Keyword
	public static selectCalender (int selectYear, String selectMonth, int selectDay, int calendarClickNumber){
		List<WebElement> selectCalender = driver.findElements(By.xpath("//div[contains(@class,'vdp-datepicker jFormGroup')]"))
		println(selectCalender.size())

		selectCalender.get(calendarClickNumber-1).click()

		WebElement btnMonth = driver.findElement(By.xpath('//div[@class="vdp-datepicker__calendar" and @style=""]//span[@class="day__month_btn up"]'))
		btnMonth.click()

		WebElement btnYear = driver.findElement(By.xpath('//div[@class="vdp-datepicker__calendar" and @style=""]//span[@class="month__year_btn up"]'))
		btnYear.click()

		WebElement cldrYear = driver.findElement(By.xpath('//div[@class="vdp-datepicker__calendar" and @style=""]//span[text()='+selectYear+']'))
		cldrYear.click()

		WebElement cldrMonth = driver.findElement(By.xpath('//div[@class="vdp-datepicker__calendar" and @style=""]//span[contains(text(),"'+selectMonth+'")]'))
		cldrMonth.click()

		WebElement cldrDay = driver.findElement(By.xpath('//div[@class="vdp-datepicker__calendar" and @style=""]//span[text()='+selectDay+']'))
		cldrDay.click()
	}
}