import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.delay(3)

WebUI.click(findTestObject('Navigations/Navigation - Starters/span_Your Starter  Leavers'))

WebUI.delay(3)

WebUI.click(findTestObject('Navigations/Navigation - Detailed Reason For Leaving/a_Leaving Reasons'))

WebUI.delay(3)

WebUI.click(findTestObject('Navigations/Navigation - Detailed Reason For Leaving/a_Detailed Reasons For Leaving'))

WebUI.delay(3)

WebUI.verifyElementPresent(findTestObject('Navigations/Navigation - Detailed Reason For Leaving/a_Add Detailed Reasons For Lea'), 
    0)

WebUI.doubleClick(findTestObject('Navigations/Navigation - Reason For Leaving/Search'))

WebUI.verifyTextPresent('Detailed Reasons For Leaving', false)

