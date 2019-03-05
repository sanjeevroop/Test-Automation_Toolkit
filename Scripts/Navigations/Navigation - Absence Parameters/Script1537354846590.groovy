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

//WebUI.waitForPageLoad(10)
WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Absence Parameters/span_Your Absence Rules'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Navigations/Navigation - Absence Parameters/a_Absence Parameters'))

//verify page title
WebUI.waitForPageLoad(2)

WebUI.verifyTextPresent('ABSENCE PARAMETERS', false)

WebUI.click(findTestObject('Navigations/Navigation - Absence Parameters/input_Holiday year start date_'))

WebUI.verifyTextPresent('Holiday year start date:', false)

